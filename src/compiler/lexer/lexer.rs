#[derive(Debug)]
pub enum AxolLexerCallbackResponse {
    Token(String),
    String(String),
    Char(char),
}

pub fn lex_source_fn<F>(source: String, mut f: F)
where
    F: FnMut(AxolLexerCallbackResponse, usize, Option<usize>),
{
    const SPECIAL_TOKENS: &[&str] = &[","];
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

                f(AxolLexerCallbackResponse::String(content), start_line, start_col);
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

                f(AxolLexerCallbackResponse::String(content), start_line, start_col);
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

        // token starter: ex. @define@define => @define + @define
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

            f(AxolLexerCallbackResponse::Token(tok.to_string()), actual_line, col);

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
    specials.iter().copied().find(|tok| starts_with_at(s, idx, tok))
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
    s.get(idx..)?.chars().next()
}

fn read_number_at(s: &str, start: usize) -> Option<(String, usize)> {
    let mut i = start;
    let mut out = String::new();
    let mut saw_digit = false;

    let first = peek_char_at(s, i)?;

    if is_ascii_digit(first) {
        // ok
    } else if first == '.' {
        let next = peek_char_at(s, i + 1)?;
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