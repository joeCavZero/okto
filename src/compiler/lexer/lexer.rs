use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::compiler::*;
use crate::core::*;
use crate::debug::*;

#[derive(Debug)]
pub enum AxolLexerCallbackResponse {
    Token(String),
    String(String),
    Char(char),
}

type OktoMacroTable = HashMap<String, (Vec<String>, Vec<OktoPositionedToken>)>;
type OktoFileTable = HashMap<usize, String>;

struct OktoProcessorContext {
    file_table: OktoFileTable,
    path_to_file_id: HashMap<String, usize>,
    next_file_id: usize,
    macro_table: OktoMacroTable,
    once_set: HashSet<String>,
    processing_stack: HashSet<String>,
}

impl OktoProcessorContext {
    fn new() -> Self {
        Self {
            file_table: HashMap::new(),
            path_to_file_id: HashMap::new(),
            next_file_id: 0,
            macro_table: HashMap::new(),
            once_set: HashSet::new(),
            processing_stack: HashSet::new(),
        }
    }

    fn get_or_create_file_id(&mut self, path: &str) -> usize {
        if let Some(id) = self.path_to_file_id.get(path) {
            return *id;
        }

        let id = self.next_file_id;
        self.next_file_id += 1;

        self.path_to_file_id.insert(path.to_string(), id);
        self.file_table.insert(id, path.to_string());

        id
    }

    fn process_virtual_tokens(
        &mut self,
        virtual_path: &str,
        mut ptokens: Vec<OktoPositionedToken>,
    ) -> Result<Vec<OktoPositionedToken>, OktoPositionedError> {
        let file_id = self.get_or_create_file_id(virtual_path);
        match self.resolve_processors_common(&virtual_path.to_string(), file_id, &mut ptokens, false) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(ptokens)
    }

    fn process_file(
        &mut self,
        file_path: &Path,
    ) -> Result<Vec<OktoPositionedToken>, OktoPositionedError> {
        let absolute_path = match canonicalize_or_fallback(file_path) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        let absolute_path_string = absolute_path.to_string_lossy().to_string();

        if self.processing_stack.contains(&absolute_path_string) {
            return Err(OktoPositionedError::new(
                format!(
                    "Include cycle detected involving '{}'",
                    absolute_path_string
                ),
                OktoPosition::new(None, 0, None),
            ));
        }

        let file_id = self.get_or_create_file_id(&absolute_path_string);
        self.processing_stack.insert(absolute_path_string.clone());

        let mut ptokens =
            match scan_positioned_tokens_from_file(&absolute_path_string, Some(file_id)) {
                Ok(v) => v,
                Err(e) => {
                    self.processing_stack.remove(&absolute_path_string);
                    return Err(e);
                }
            };

        if let Err(e) =
            self.resolve_processors_common(&absolute_path_string, file_id, &mut ptokens, true)
        {
            self.processing_stack.remove(&absolute_path_string);
            return Err(e);
        }

        self.processing_stack.remove(&absolute_path_string);
        Ok(ptokens)
    }

    fn resolve_processors_common(
        &mut self,
        absolute_file_path: &String,
        _file_id: usize,
        ptokens: &mut Vec<OktoPositionedToken>,
        file_mode: bool,
    ) -> Result<(), OktoPositionedError> {
        let mut token_counter: usize = 0;

        while token_counter < ptokens.len() {
            let current = match ptokens.get(token_counter).cloned() {
                Some(v) => v,
                None => break,
            };

            match current.token.clone() {
                OktoToken::Processor(OktoProcessor::Include) => {
                    if !file_mode {
                        return Err(OktoPositionedError::new(
                            "You should not use includes in this mode".to_string(),
                            current.position.clone(),
                        ));
                    }

                    let next = match ptokens.get(token_counter + 1).cloned() {
                        Some(v) => v,
                        None => {
                            return Err(OktoPositionedError::new(
                                "Include directive must be followed by a file path".to_string(),
                                current.position.clone(),
                            ));
                        }
                    };

                    let include_literal = match next.token.clone() {
                        OktoToken::Literal(OktoLiteral::String(s)) => s,
                        _ => {
                            return Err(OktoPositionedError::new(
                                "Include directive must be followed by a string literal"
                                    .to_string(),
                                next.position.clone(),
                            ));
                        }
                    };

                    let resolved_include_path =
                        resolve_include_path(absolute_file_path, &include_literal);

                    let included_tokens = match self.process_file(&resolved_include_path) {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };

                    ptokens.remove(token_counter); // @include
                    ptokens.remove(token_counter); // "file"

                    for included in included_tokens.into_iter().rev() {
                        ptokens.insert(token_counter, included);
                    }

                    continue;
                }

                OktoToken::Processor(OktoProcessor::Macro) => {
                    let name_token = match ptokens.get(token_counter + 1).cloned() {
                        Some(v) => v,
                        None => {
                            return Err(OktoPositionedError::new(
                                "Macro directive must be followed by an identifier".to_string(),
                                current.position.clone(),
                            ));
                        }
                    };

                    let macro_name = match identifier_name(&name_token.token) {
                        Some(s) => s.to_string(),
                        None => {
                            return Err(OktoPositionedError::new(
                                "Macro directive must be followed by an identifier".to_string(),
                                name_token.position.clone(),
                            ));
                        }
                    };

                    let (macro_args, head_consumed) =
                        match collect_macro_definition_args(ptokens, token_counter + 2) {
                            Ok((args, body)) => (args, body),
                            Err(e) => {
                                return Err(OktoPositionedError::new(
                                    format!("Error parsing macro definition: {}", e.error),
                                    name_token.position.clone(),
                                ));
                            }
                        };

                    let body_start = token_counter + 2 + head_consumed;
                    let (macro_body, body_consumed) =
                        collect_macro_body(ptokens, body_start, name_token.position.line);

                    self.macro_table
                        .insert(macro_name, (macro_args, macro_body));

                    let total_to_remove = 2 + head_consumed + body_consumed;
                    for _ in 0..total_to_remove {
                        if token_counter < ptokens.len() {
                            ptokens.remove(token_counter);
                        }
                    }

                    continue;
                }

                OktoToken::Processor(OktoProcessor::Once) => {
                    if self.once_set.contains(absolute_file_path) {
                        ptokens.truncate(token_counter);
                        continue;
                    } else {
                        self.once_set.insert(absolute_file_path.clone());
                        ptokens.remove(token_counter);
                        continue;
                    }
                }

                _ => {
                    if let Some(name) = identifier_name(&current.token) {
                        if let Some((macro_args, macro_body)) = self.macro_table.get(name).cloned()
                        {
                            let (call_args, head_consumed) = match collect_macro_call_args(
                                ptokens,
                                token_counter + 1,
                                &current.position,
                            ) {
                                Ok(v) => v,
                                Err(e) => {
                                    return Err(OktoPositionedError::new(
                                        format!("Error parsing macro call: {}", e.error),
                                        current.position.clone(),
                                    ));
                                }
                            };

                            if call_args.len() != macro_args.len() {
                                return Err(OktoPositionedError::new(
                                    format!(
                                        "Macro '{}' called with incorrect number of arguments",
                                        name
                                    ),
                                    current.position.clone(),
                                ));
                            }

                            let mut substituted = macro_body.clone();

                            for body_ptkn in substituted.iter_mut() {
                                if let Some(param_name) = macro_param_name(&body_ptkn.token) {
                                    let arg_index = match macro_args
                                        .iter()
                                        .position(|p| *p == param_name)
                                    {
                                        Some(i) => i,
                                        None => {
                                            return Err(OktoPositionedError::new(
                                                format!(
                                                    "Macro argument '{}' not associated with any argument in the macro definition head",
                                                    param_name
                                                ),
                                                body_ptkn.position.clone(),
                                            ));
                                        }
                                    };

                                    *body_ptkn = call_args[arg_index].clone();
                                }
                            }

                            for body_ptkn in substituted.iter() {
                                if let Some(param_name) = macro_param_name(&body_ptkn.token) {
                                    return Err(OktoPositionedError::new(
                                        format!(
                                            "Macro argument '{}' not associated with any argument in the macro definition head",
                                            param_name
                                        ),
                                        body_ptkn.position.clone(),
                                    ));
                                }
                            }

                            let remove_count = 1 + head_consumed;
                            for _ in 0..remove_count {
                                if token_counter < ptokens.len() {
                                    ptokens.remove(token_counter);
                                }
                            }

                            for new_ptkn in substituted.into_iter().rev() {
                                ptokens.insert(token_counter, new_ptkn);
                            }

                            continue;
                        }
                    }

                    token_counter += 1;
                }
            }
        }

        Ok(())
    }
}

pub fn lex_and_process_file(
    file: &String,
) -> Result<(Vec<OktoPositionedToken>, OktoFileTable), OktoPositionedError> {
    let mut ctx = OktoProcessorContext::new();
    match ctx.process_file(Path::new(file)) {
        Ok(ptkns) => return Ok((ptkns, ctx.file_table,)),
        Err(e) => return Err(e),
    }
}

pub fn process_tokens(
    positioned_tokens: Vec<OktoPositionedToken>,
) -> Result<Vec<OktoPositionedToken>, OktoPositionedError> {
    let mut ctx = OktoProcessorContext::new();
    ctx.process_virtual_tokens("<memory>", positioned_tokens)
}

fn canonicalize_or_fallback(path: &Path) -> Result<PathBuf, OktoPositionedError> {
    match std::fs::canonicalize(path) {
        Ok(p) => Ok(p),
        Err(e) => Err(OktoPositionedError::new(
            format!("Failed to read file '{}': {}", path.display(), e),
            OktoPosition::new(None, 0, None),
        )),
    }
}

fn resolve_include_path(including_file_path: &str, include_literal: &str) -> PathBuf {
    let include_path = PathBuf::from(include_literal);

    if include_path.is_absolute() {
        return include_path;
    }

    let including_parent = Path::new(including_file_path)
        .parent()
        .unwrap_or(Path::new("."));

    including_parent.join(include_path)
}

fn identifier_name(token: &OktoToken) -> Option<&str> {
    match token {
        OktoToken::Identifier(s) => Some(s.as_str()),
        _ => None,
    }
}

fn is_backslash_token(token: &OktoToken) -> bool {
    match token {
        OktoToken::Identifier(s) if s == "\\" => true,
        _ => false,
    }
}


fn collect_macro_body(
    ptokens: &[OktoPositionedToken],
    start_index: usize,
    macro_line: usize,
) -> (Vec<OktoPositionedToken>, usize) {
    let mut body = Vec::new();
    let mut i = start_index;
    let mut current_line = macro_line;
    let mut continue_to_next_line = false;

    while i < ptokens.len() {
        let ptkn = &ptokens[i];

        if ptkn.position.line != current_line {
            if continue_to_next_line {
                continue_to_next_line = false;
                current_line = ptkn.position.line;
            } else {
                break;
            }
        }

        if is_backslash_token(&ptkn.token) {
            continue_to_next_line = true;
            i += 1;
            continue;
        }

        body.push(ptkn.clone());
        i += 1;
    }

    (body, i - start_index)
}

fn scan_positioned_tokens_from_file(
    file_path: &str,
    file_id: Option<usize>,
) -> Result<Vec<OktoPositionedToken>, OktoPositionedError> {
    match std::fs::read_to_string(file_path) {
        Ok(src) => lex_source(&src, file_id),
        Err(e) => Err(OktoPositionedError::new(
            format!("Failed to read file '{}': {}", file_path, e),
            OktoPosition::new(file_id, 0, None),
        )),
    }
}

fn lex_source(
    source: &String,
    file_id: Option<usize>,
) -> Result<Vec<OktoPositionedToken>, OktoPositionedError> {
    let mut positioned_tokens = Vec::new();
    let mut err = None;

    lex_source_fn(source, |res, line, column| match res {
        AxolLexerCallbackResponse::Token(t) => match OktoToken::from(&t) {
            Ok(tt) => positioned_tokens.push(OktoPositionedToken::new(
                tt,
                OktoPosition::new(file_id, line, column),
            )),
            Err(e) => {
                err = Some(OktoPositionedError::new(
                    e,
                    OktoPosition::new(file_id, line, column),
                ));
            }
        },

        AxolLexerCallbackResponse::String(s) => {
            positioned_tokens.push(OktoPositionedToken::new(
                OktoToken::new_string_literal(&s),
                OktoPosition::new(file_id, line, column),
            ));
        }

        AxolLexerCallbackResponse::Char(c) => {
            positioned_tokens.push(OktoPositionedToken::new(
                OktoToken::new_char_literal(&c),
                OktoPosition::new(file_id, line, column),
            ));
        }
    });

    if let Some(e) = err {
        return Err(e);
    }

    Ok(positioned_tokens)
}

fn lex_source_fn<F>(source: &String, mut f: F)
where
    F: FnMut(AxolLexerCallbackResponse, usize, Option<usize>),
{
    const SPECIAL_TOKENS: &[&str] = &[",", "\\", "(", ")"];
    const TOKEN_ENDERS: &[&str] = &[":"];
    const TOKEN_STARTERS: &[&str] = &["@"];

    const STRING_SEPARATORS: &[(&str, &str)] = &[("\"", "\""), ("#\"", "\"#")];
    const CHAR_SEPARATORS: &[(&str, &str)] = &[("'", "'")];

    const LINE_COMMENT: &str = "//";
    const MULTI_LINE_COMMENT: (&str, &str) = ("/*", "*/");
    const SPACE: char = ' ';
    const NEW_LINE: char = '\n';
    const ESCAPE_CHARACTER: char = '\\';

    let mut special_tokens = SPECIAL_TOKENS.to_vec();
    let mut token_enders = TOKEN_ENDERS.to_vec();
    let mut token_starters = TOKEN_STARTERS.to_vec();
    let mut string_separators = STRING_SEPARATORS.to_vec();
    let mut char_separators = CHAR_SEPARATORS.to_vec();

    special_tokens.sort_by(|a, b| b.len().cmp(&a.len()));
    token_enders.sort_by(|a, b| b.len().cmp(&a.len()));
    token_starters.sort_by(|a, b| b.len().cmp(&a.len()));
    string_separators.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    char_separators.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    let src = source.replace('\r', "");
    let bytes_len = src.len();

    let mut i: usize = 0;

    let mut actual_line: usize = 1;
    let mut actual_column: Option<usize> = Some(1);
    let mut line_has_tab_indent = false;

    let mut token_acc = String::new();
    let mut token_start_column: Option<usize> = Some(1);

    while i < bytes_len {
        let ch = peek_char_at(&src, i).unwrap_or('\0');
        let ch_len = ch.len_utf8();

        if token_acc.is_empty() {
            token_start_column = actual_column;
        }

        // tab
        if ch == '\t' {
            flush_token(
                &mut token_acc,
                token_start_column,
                line_has_tab_indent,
                actual_line,
                &mut f,
            );
            line_has_tab_indent = true;
            actual_column = None;
            i += ch_len;
            continue;
        }

        // newline
        if ch == NEW_LINE {
            flush_token(
                &mut token_acc,
                token_start_column,
                line_has_tab_indent,
                actual_line,
                &mut f,
            );
            token_start_column = Some(1);
            actual_line += 1;
            actual_column = Some(1);
            line_has_tab_indent = false;
            i += ch_len;
            continue;
        }

        // space
        if ch == SPACE {
            flush_token(
                &mut token_acc,
                token_start_column,
                line_has_tab_indent,
                actual_line,
                &mut f,
            );
            bump_col(&mut actual_column, 1);
            i += ch_len;
            continue;
        }

        // line comment
        if starts_with_at(&src, i, LINE_COMMENT) {
            flush_token(
                &mut token_acc,
                token_start_column,
                line_has_tab_indent,
                actual_line,
                &mut f,
            );

            i += LINE_COMMENT.len();
            bump_col(&mut actual_column, LINE_COMMENT.len());

            while i < bytes_len {
                let c = peek_char_at(&src, i).unwrap_or('\0');
                let l = c.len_utf8();
                i += l;

                if c == '\t' {
                    line_has_tab_indent = true;
                    actual_column = None;
                } else if c == NEW_LINE {
                    actual_line += 1;
                    actual_column = Some(1);
                    line_has_tab_indent = false;
                    break;
                } else {
                    bump_col(&mut actual_column, 1);
                }
            }

            continue;
        }

        // multiline comment
        if starts_with_at(&src, i, MULTI_LINE_COMMENT.0) {
            flush_token(
                &mut token_acc,
                token_start_column,
                line_has_tab_indent,
                actual_line,
                &mut f,
            );

            let start = i;
            i += MULTI_LINE_COMMENT.0.len();

            while i < bytes_len {
                if starts_with_at(&src, i, MULTI_LINE_COMMENT.1) {
                    i += MULTI_LINE_COMMENT.1.len();
                    break;
                }

                let c = peek_char_at(&src, i).unwrap_or('\0');
                i += c.len_utf8();
            }

            let consumed = &src[start..i.min(bytes_len)];
            advance_position_by_str(
                consumed,
                &mut actual_line,
                &mut actual_column,
                &mut line_has_tab_indent,
            );
            continue;
        }

        // string literal
        if let Some((open, close)) = find_separator_at(&src, i, &string_separators) {
            flush_token(
                &mut token_acc,
                token_start_column,
                line_has_tab_indent,
                actual_line,
                &mut f,
            );

            let start_line = actual_line;
            let start_col = if line_has_tab_indent {
                None
            } else {
                actual_column
            };

            if let Some(end) = read_delimited_end(&src, i, open, close, ESCAPE_CHARACTER) {
                let consumed = &src[i..end];
                let inner = &src[i + open.len()..end - close.len()];
                let content = unescape_string(inner, ESCAPE_CHARACTER);

                advance_position_by_str(
                    consumed,
                    &mut actual_line,
                    &mut actual_column,
                    &mut line_has_tab_indent,
                );

                f(
                    AxolLexerCallbackResponse::String(content),
                    start_line,
                    start_col,
                );
                i = end;
                continue;
            } else {
                let consumed = &src[i..];
                let inner = &src[i + open.len()..];
                let content = unescape_string(inner, ESCAPE_CHARACTER);

                advance_position_by_str(
                    consumed,
                    &mut actual_line,
                    &mut actual_column,
                    &mut line_has_tab_indent,
                );

                f(
                    AxolLexerCallbackResponse::String(content),
                    start_line,
                    start_col,
                );
                break;
            }
        }

        // char literal
        if let Some((open, close)) = find_separator_at(&src, i, &char_separators) {
            flush_token(
                &mut token_acc,
                token_start_column,
                line_has_tab_indent,
                actual_line,
                &mut f,
            );

            let start_line = actual_line;
            let start_col = if line_has_tab_indent {
                None
            } else {
                actual_column
            };

            if let Some(end) = read_delimited_end(&src, i, open, close, ESCAPE_CHARACTER) {
                let consumed = &src[i..end];
                let inner = if end >= i + open.len() + close.len() {
                    &src[i + open.len()..end - close.len()]
                } else {
                    ""
                };

                let c = extract_char_inner(inner, ESCAPE_CHARACTER);

                advance_position_by_str(
                    consumed,
                    &mut actual_line,
                    &mut actual_column,
                    &mut line_has_tab_indent,
                );

                f(AxolLexerCallbackResponse::Char(c), start_line, start_col);
                i = end;
                continue;
            } else {
                f(AxolLexerCallbackResponse::Char('\0'), start_line, start_col);
                break;
            }
        }

        // number
        if token_acc.is_empty() {
            if let Some((num, consumed)) = read_number_at(&src, i) {
                let col = if line_has_tab_indent {
                    None
                } else {
                    actual_column
                };

                f(AxolLexerCallbackResponse::Token(num), actual_line, col);
                i += consumed;
                bump_col_by_str(&mut actual_column, &src[i - consumed..i]);
                continue;
            }
        }

        // token starter: ex. @macro@macro => @macro + @macro
        if token_acc.is_empty() {
            if let Some(starter) = find_special_at(&src, i, &token_starters) {
                let start = i;
                i += starter.len();

                while i < bytes_len {
                    if is_token_boundary(&src, i)
                        || find_special_at(&src, i, &token_starters).is_some()
                        || find_special_at(&src, i, &special_tokens).is_some()
                    {
                        break;
                    }

                    if let Some(ender) = find_special_at(&src, i, &token_enders) {
                        i += ender.len();
                        break;
                    }

                    let c = peek_char_at(&src, i).unwrap_or('\0');
                    i += c.len_utf8();
                }

                let tok = src[start..i].to_string();
                let col = if line_has_tab_indent {
                    None
                } else {
                    actual_column
                };
                f(AxolLexerCallbackResponse::Token(tok), actual_line, col);
                bump_col_by_str(&mut actual_column, &src[start..i]);
                continue;
            }
        }

        // special token
        if let Some(tok) = find_special_at(&src, i, &special_tokens) {
            flush_token(
                &mut token_acc,
                token_start_column,
                line_has_tab_indent,
                actual_line,
                &mut f,
            );

            let col = if line_has_tab_indent {
                None
            } else {
                actual_column
            };

            f(
                AxolLexerCallbackResponse::Token(tok.to_string()),
                actual_line,
                col,
            );

            i += tok.len();
            bump_col_by_str(&mut actual_column, tok);
            continue;
        }

        // token ender: ex. label:add => label: + add
        token_acc.push(ch);
        i += ch_len;
        bump_col(&mut actual_column, 1);

        if let Some(ender) = find_suffix_in_acc(&token_acc, &token_enders) {
            let tok = std::mem::take(&mut token_acc);
            let col = if line_has_tab_indent {
                None
            } else {
                token_start_column
            };

            f(AxolLexerCallbackResponse::Token(tok), actual_line, col);

            let _ = ender;
            token_start_column = actual_column;
        }
    }

    flush_token(
        &mut token_acc,
        token_start_column,
        line_has_tab_indent,
        actual_line,
        &mut f,
    );
}

fn is_token_boundary(s: &str, idx: usize) -> bool {
    match peek_char_at(s, idx) {
        Some(' ') | Some('\n') | Some('\t') => true,
        Some(_) => false,
        None => true,
    }
}

fn bump_col(col: &mut Option<usize>, n: usize) {
    if let Some(c) = *col {
        *col = Some(c + n);
    }
}

fn bump_col_by_str(col: &mut Option<usize>, s: &str) {
    if let Some(c) = *col {
        *col = Some(c + s.chars().count());
    }
}

fn starts_with_at(s: &str, idx: usize, pat: &str) -> bool {
    s.as_bytes()
        .get(idx..idx + pat.len())
        .map(|b| b == pat.as_bytes())
        .unwrap_or(false)
}

fn find_special_at<'a>(s: &str, idx: usize, specials: &'a [&'a str]) -> Option<&'a str> {
    specials
        .iter()
        .copied()
        .find(|tok| starts_with_at(s, idx, tok))
}

fn find_suffix_in_acc<'a>(acc: &str, suffixes: &'a [&'a str]) -> Option<&'a str> {
    suffixes.iter().copied().find(|suf| acc.ends_with(suf))
}

fn find_separator_at<'a>(
    s: &str,
    idx: usize,
    seps: &'a [(&'a str, &'a str)],
) -> Option<(&'a str, &'a str)> {
    seps.iter()
        .copied()
        .find(|(open, _)| starts_with_at(s, idx, open))
}

fn read_delimited_end(
    s: &str,
    start: usize,
    open: &str,
    close: &str,
    escape: char,
) -> Option<usize> {
    if !starts_with_at(s, start, open) {
        return None;
    }

    let mut i = start + open.len();
    let mut escaped = false;

    while i < s.len() {
        let ch = peek_char_at(s, i).unwrap_or('\0');
        let l = ch.len_utf8();

        if escaped {
            escaped = false;
            i += l;
            continue;
        }

        if ch == escape {
            escaped = true;
            i += l;
            continue;
        }

        if starts_with_at(s, i, close) {
            return Some(i + close.len());
        }

        i += l;
    }

    None
}

fn advance_position_by_str(
    consumed: &str,
    line: &mut usize,
    col: &mut Option<usize>,
    line_has_tab_indent: &mut bool,
) {
    for ch in consumed.chars() {
        if ch == '\n' {
            *line += 1;
            *col = Some(1);
            *line_has_tab_indent = false;
        } else if ch == '\t' {
            *line_has_tab_indent = true;
            *col = None;
        } else {
            bump_col(col, 1);
        }
    }
}

fn flush_token<F>(
    token_acc: &mut String,
    token_start_column: Option<usize>,
    line_has_tab_indent: bool,
    actual_line: usize,
    f: &mut F,
) where
    F: FnMut(AxolLexerCallbackResponse, usize, Option<usize>),
{
    if token_acc.is_empty() {
        return;
    }

    let col = if line_has_tab_indent {
        None
    } else {
        token_start_column
    };

    let tok = std::mem::take(token_acc);
    f(AxolLexerCallbackResponse::Token(tok), actual_line, col);
}

fn extract_char_inner(inner: &str, escape: char) -> char {
    let mut it = inner.chars();
    let first = it.next().unwrap_or('\0');

    if first != escape {
        return first;
    }

    match it.next().unwrap_or('\0') {
        'n' => '\n',
        't' => '\t',
        'r' => '\r',
        '\\' => '\\',
        '"' => '"',
        '\'' => '\'',
        other => other,
    }
}

fn unescape_string(s: &str, escape: char) -> String {
    let mut out = String::new();
    let mut it = s.chars();

    while let Some(ch) = it.next() {
        if ch != escape {
            out.push(ch);
            continue;
        }

        match it.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some('\'') => out.push('\''),
            Some(other) => out.push(other),
            None => break,
        }
    }

    out
}

fn is_ascii_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn peek_char_at(s: &str, idx: usize) -> Option<char> {
    match s.get(idx..) {
        Some(sub) => sub.chars().next(),
        None => None,
    }
}

fn read_number_at(s: &str, start: usize) -> Option<(String, usize)> {
    let mut i = start;
    let mut out = String::new();
    let mut saw_digit = false;

    let first = match peek_char_at(s, i) {
        Some(ch) => ch,
        None => return None,
    };

    if is_ascii_digit(first) {
        // ok
    } else if first == '.' {
        let next = match peek_char_at(s, i + 1) {
            Some(ch) => ch,
            None => return None,
        };
        if !is_ascii_digit(next) {
            return None;
        }
    } else {
        return None;
    }

    while let Some(ch) = peek_char_at(s, i) {
        if is_ascii_digit(ch) {
            saw_digit = true;
            out.push(ch);
            i += ch.len_utf8();
        } else {
            break;
        }
    }

    if let Some('.') = peek_char_at(s, i) {
        out.push('.');
        i += 1;

        while let Some(ch) = peek_char_at(s, i) {
            if is_ascii_digit(ch) {
                saw_digit = true;
                out.push(ch);
                i += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    if let Some(ch) = peek_char_at(s, i) {
        if ch == 'e' || ch == 'E' {
            if !saw_digit {
                return None;
            }

            out.push(ch);
            i += ch.len_utf8();

            if let Some(sign) = peek_char_at(s, i) {
                if sign == '+' || sign == '-' {
                    out.push(sign);
                    i += sign.len_utf8();
                }
            }

            let mut exp_digits = 0;
            while let Some(d) = peek_char_at(s, i) {
                if is_ascii_digit(d) {
                    out.push(d);
                    i += d.len_utf8();
                    exp_digits += 1;
                } else {
                    break;
                }
            }

            if exp_digits == 0 {
                return None;
            }
        }
    }

    if !saw_digit || out == "." {
        return None;
    }

    if let Some(suf) = peek_char_at(s, i) {
        if is_number_suffix(suf) {
            out.push(suf);
            i += suf.len_utf8();
        }
    }

    Some((out, i - start))
}

fn is_number_suffix(ch: char) -> bool {
    matches!(ch, 'u' | 'i' | 'b' | 'f')
}

fn is_open_paren_token(token: &OktoToken) -> bool {
    match token {
        OktoToken::Identifier(s) => s == "(",
        _ => false,
    }
}

fn is_close_paren_token(token: &OktoToken) -> bool {
    match token {
        OktoToken::Identifier(s) => s == ")",
        _ => false,
    }
}

fn is_comma_token(token: &OktoToken) -> bool {
    match token {
        OktoToken::Identifier(s) => s == ",",
        _ => false,
    }
}

fn macro_param_name(token: &OktoToken) -> Option<String> {
    match token {
        OktoToken::MacroArg(s) => Some(s.clone()),
        _ => None,
    }
}

fn collect_macro_definition_args(
    ptokens: &[OktoPositionedToken],
    start_index: usize,
) -> Result<(Vec<String>, usize), OktoPositionedError> {
    let Some(first) = ptokens.get(start_index) else {
        return Ok((Vec::new(), 0));
    };

    if !is_open_paren_token(&first.token) {
        return Ok((Vec::new(), 0));
    }

    let mut args = Vec::new();
    let mut i = start_index + 1;
    let mut expect_arg = true;

    while i < ptokens.len() {
        let ptkn = &ptokens[i];

        if is_close_paren_token(&ptkn.token) {
            return Ok((args, i - start_index + 1));
        }

        if expect_arg {
            match macro_param_name(&ptkn.token) {
                Some(name) => {
                    args.push(name.to_string());
                    expect_arg = false;
                    i += 1;
                }
                None => {
                    return Err(
                        OktoPositionedError::new(
                            "Expected macro argument in definition head".to_string(),
                            ptkn.position.clone(),
                        )
                    )
                }
            }
        } else {
            if is_comma_token(&ptkn.token) {
                expect_arg = true;
                i += 1;
            } else {
                return Err(
                    OktoPositionedError::new(
                        "Expected ',' or ')' in macro definition head".to_string(),
                        ptkn.position.clone(),
                    )
                );
            }
        }
    }

    Err(
        OktoPositionedError::new(
            "Unclosed macro definition head".to_string(),
            first.position.clone(),
        )
    )
}

fn collect_macro_call_args(
    ptokens: &[OktoPositionedToken],
    start_index: usize,
    fallback_pos: &OktoPosition,
) -> Result<(Vec<OktoPositionedToken>, usize), OktoPositionedError> {
    let Some(first) = ptokens.get(start_index) else {
        return Ok((Vec::new(), 0));
    };

    if !is_open_paren_token(&first.token) {
        return Ok((Vec::new(), 0));
    }

    let mut args: Vec<OktoPositionedToken> = Vec::new();
    let mut i = start_index + 1;
    let mut expect_arg = true;

    while i < ptokens.len() {
        let ptkn = &ptokens[i];

        if is_close_paren_token(&ptkn.token) {
            if expect_arg && !args.is_empty() {
                return Err(
                    OktoPositionedError::new(
                        "Trailing comma in macro call".to_string(),
                        ptkn.position.clone(),
                    )
                );
            }

            return Ok((args, i - start_index + 1));
        }

        if expect_arg {
            if is_comma_token(&ptkn.token) {
                return Err(
                    OktoPositionedError::new(
                        "Expected macro argument".to_string(),
                        ptkn.position.clone(),
                    )
                );
            }

            args.push(ptkn.clone());
            expect_arg = false;
            i += 1;
        } else {
            if is_comma_token(&ptkn.token) {
                expect_arg = true;
                i += 1;
            } else {
                return Err(
                    OktoPositionedError::new(
                        "Expected ',' or ')' in macro call".to_string(),
                        ptkn.position.clone(),
                    )
                );
            }
        }
    }

    Err(
        OktoPositionedError::new(
            "Unclosed macro call head".to_string(),
            fallback_pos.clone(),
        )
    )
}