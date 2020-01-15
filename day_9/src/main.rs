
fn main() {
    let data = load_input();
    let routes = parse_input(data);
    
}

#[derive(Debug, Hash)]
struct Route {
    from: String,
    to: String,
    distance: u32,
}

impl Route {
    fn new(from: &str, to: &str, distance: &str) -> Self {
        let dist = match distance.parse::<u32>() {
            Ok(val) => val,
            Err(e) => panic!(e),
        };

        Route {
            from: from.to_string(),
            to: to.to_string(),
            distance: dist,
        }
    }
}

fn load_input() -> &'static str {
    include_str!("input.txt")
}

fn parse_input(input: &str) -> Vec<Route> {
    let route_opts = input
        .split("\n")
        .filter(|i| !i.is_empty())
        .collect::<Vec<&str>>();

    let mut routes = Vec::new();

    for opt in route_opts {
        let line = opt.split_whitespace().collect::<Vec<&str>>();
        let route = Route::new(line[0], line[2], line[4]);
        routes.push(route);
    }

    routes
}
