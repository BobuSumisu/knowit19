fn main() {
    let input = "tMlsioaplnKlflgiruKanliaebeLlkslikkpnerikTasatamkDpsdakeraBeIdaegptnuaKtmteorpuTaTtbtsesOHXxonibmksekaaoaKtrssegnveinRedlkkkroeekVtkekymmlooLnanoKtlstoepHrpeutdynfSneloietbol";
    let result = solve(input);
    println!("{}", result);
}

fn solve(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    let n = chars.len();
    let h = n / 2;

    for i in (0..h).step_by(3) {
        chars.swap(i, n - i - 3);
        chars.swap(i + 1, n - i - 2);
        chars.swap(i + 2, n - i - 1);
    }

    for i in (0..(n - 1)).step_by(2) {
        chars.swap(i, i + 1);
    }

    for i in 0..h {
        chars.swap(i, h + i);
    }

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
}
