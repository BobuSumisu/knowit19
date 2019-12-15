#[derive(Copy, Clone, Debug)]
struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

impl Vertex {
    const fn from_slice(values: &[f64]) -> Self {
        Self {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }

    fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

fn parse_input(input: &str) -> Vec<(Vertex, Vertex, Vertex)> {
    input
        .lines()
        .map(|l| {
            let v: Vec<f64> = l.split(',').map(|x| x.parse().unwrap()).collect();
            (
                Vertex::from_slice(&v[0..3]),
                Vertex::from_slice(&v[3..6]),
                Vertex::from_slice(&v[6..9]),
            )
        })
        .collect()
}

fn calculate_volume(vertices: &[(Vertex, Vertex, Vertex)]) -> f64 {
    vertices
        .iter()
        .map(|(v1, v2, v3)| v1.dot(v2.cross(*v3)))
        .sum::<f64>()
        / 6.0
}

fn solution() -> f64 {
    let vertices = parse_input(include_str!("../input/input.txt"));
    let volume = calculate_volume(&vertices) * 0.001;
    (volume * 1000.0).round() / 1000.0
}

fn main() {
    println!("{}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!((solution() - 7.302).abs() < std::f64::EPSILON);
    }
}
