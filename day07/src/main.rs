
#[derive(Clone, Debug, Default)]
struct Dir {
    files: Vec<(String, u64)>,
    dirs: Vec<(String, Dir)>,
}

impl Dir {

    fn get_dir(&mut self, name: &str) -> Option<&mut Dir> {
        self.dirs.iter_mut().find(|(n, _)| n == name).map(|(_, k)| k)
    }

    fn fill_dirs<'i,'s,I>(&mut self, lines: &'i mut I)
        where I: Iterator<Item = &'s str>
    {
        while let Some(line) = lines.next() {
            if line == "$ cd .." { break; }
            if let Some(name) = line.strip_prefix("$ cd ") {
                let name = name.trim_end();
                self.get_dir(name).unwrap().fill_dirs(lines);
            }
            if line.starts_with('$') { continue; }
            let (val, name) = line.split_once(' ').unwrap();
            if val == "dir" {
                self.dirs.push((String::from(name), Dir::default()));
            }
            else {
                self.files.push((String::from(name), val.parse().unwrap()));
            }
        }
    }

    fn file_sum(&self) -> u64 {
        self.files.iter().map(|(_,v)| v).sum()
    }

    fn part1_sums(&self, limit: u64) -> (u64, u64) {
        let (part1_sum, content_sum) = self.dirs.iter()
            .map(|(_,d)| {
                d.part1_sums(limit)
            })
            .fold((0,0), |acc, val| (acc.0+val.0, acc.1+val.1));
        let content_sum = self.file_sum() + content_sum;
        let part1_sum = part1_sum + if content_sum <= limit { content_sum } else { 0 };
        (part1_sum, content_sum)
    }

    fn size(&self) -> u64 {
        self.files.iter().map(|(_,v)| v).sum::<u64>()
            + self.dirs.iter().map(|(_,d)| d.size()).sum::<u64>()
    }

    fn find_smallest_over_val(&self, size: u64) -> (Option<(&str, u64)>, u64) {
        let (smallest, content_sum) = self.dirs.iter()
            .map(|(n, d)| {
                (n.as_str(), d.find_smallest_over_val(size))
            })
            .fold((None, 0), |acc: (Option<(&str, u64)>, u64), val: (&str, (Option<(&str, u64)>, u64))| {
                // Well this is, uh, goofy. We made rust-analyzer sad
                let smallest = match (acc.0, val.1.0) {
                    (None, None) => if val.1.1 >= size { Some((val.0,val.1.1)) } else { None },
                    (Some(x), None) => if val.1.1 >= size && val.1.1 < x.1 { Some((val.0,val.1.1)) } else { Some(x) },
                    (None, Some(x)) => Some(x),
                    (Some(x), Some(y)) => if x.1 <= y.1 { Some(x) } else { Some(y) },
                };
                (smallest, acc.1 + val.1.1)
            });

        (smallest, self.file_sum() + content_sum)
    }

    fn print(&self) {
        self.print_indent(0)
    }

    fn print_indent(&self, indent: u32) {
        for (name, dir) in self.dirs.iter() {
            for _ in 0..indent { print!(" "); }
            println!("- {} (dir)", name);
            dir.print_indent(indent+2);
        }
        for (name, size) in self.files.iter() {
            for _ in 0..indent { print!(" "); }
            println!("- {} (file, size={})", name, size);
        }
    }

}



fn main() {
    let input = include_str!("input");

    let mut root = Dir::default();

    root.fill_dirs(&mut input.lines().skip(1));
    root.print();

    println!("Part 1 Solution: {}", root.part1_sums(100000).0);

    let needed_space = root.size() - 40000000;

    let smallest = root.find_smallest_over_val(needed_space).0.unwrap();
    println!("Part 2 Solution: {} - {}", smallest.0, smallest.1);

}
