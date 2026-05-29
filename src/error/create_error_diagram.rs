/// Extracts the line, line no and col no from the program and the line the error occurred at
pub fn get_line(program: &str, index: usize) -> (&str, usize, usize) {
    let start = program[..index]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0);

    let end = program[index..]
        .find('\n')
        .map(|i| index + i)
        .unwrap_or(program.len());

    let line_no = program[..index].matches("\n").count() + 1;
    let column_no = index - start;

    (&program[start..end], line_no, column_no)
}

/// Gets the next line after the error line. Returns an option as the error line could be the final line
pub fn get_next_line(program: &str, index: usize) -> Option<&str> {
    if index >= program.len() {
        None
    } else {
        let (line, _, _) = get_line(program, index);
        Some(line)
    }
}

///
pub fn get_prev_line(program: &str, index: isize) -> Option<&str> {
    if index <= 0 {
        None
    } else {
        let (line, _, _) = get_line(program, index as usize);
        Some(line)
    }
}

/// Creates a diagram with an arrow pointing at the line and column the error occurred in
pub fn create_error_diagram(line: &str, next_line: Option<&str>, prev_line: Option<&str>, line_no: &usize, column_no: &usize) -> String {

    let gap = line_no / 10 + 2;

    let error_line = format!("{}--> filepath:{}:{}", " ".repeat(gap-2), line_no, column_no);

    let prev_line = match prev_line {
        Some(prev_line) => {
            if *column_no == 0 {
                let prev_line = prev_line.trim_end_matches('\r');
                if !prev_line.ends_with(';') {
                    let str1 = format!("{}{}|     {}\n", line_no-1, " ".repeat(gap - line_no / 10 - 1), prev_line);
                    let str2 = format!("{}|     {}^ help: add ';' here\n", " ".repeat(gap), " ".repeat(prev_line.len()));

                    format!("{}{}", str1, str2)
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            }
        },
        None => {"".to_string()}
    };

    let first_line = format!("{}|", " ".repeat(gap));
    let second_line = format!("{}{}|     {}", line_no, " ".repeat(gap - line_no / 10 - 1), line);
    let third_line = format!("{}|  ___{}^", " ".repeat(gap), "_".repeat(*column_no));

    let fourth_line = match next_line {
        Some(next_line) => {
            if prev_line == String::from("") {
                format!("{}{}| |   {}", line_no+1, " ".repeat(gap - (line_no+1) / 10 - 1), next_line)
            } else {
                format!("{}| |", " ".repeat(gap))
            }
        },
        None => format!("{}| |", " ".repeat(gap))
    };

    format!("{}\n{}\n{}{}\n{}\n{}\n", error_line, first_line, prev_line, second_line, third_line, fourth_line)
}