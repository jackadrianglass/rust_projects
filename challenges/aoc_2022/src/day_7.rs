use std::iter::Peekable;

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug, PartialEq)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Box<Self>>,
    _size: Option<i32>,
}

fn parse_contents(lines: &[&str]) -> Dir {
    let mut dir = Dir::new("/");
    let mut iter = lines.iter().map(|s| *s);
    iter.next();
    dir.build(&mut iter.peekable());
    dir
}

impl Dir {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
            dirs: Vec::new(),
            _size: None,
        }
    }

    fn build<'a>(&mut self, lines: &mut Peekable<impl Iterator<Item = &'a str>>) {
        loop {
            let line = lines.next();
            if let None = line {
                break;
            }
            let contents = line.unwrap();

            if contents.starts_with("$ ls") {
                loop {
                    if let Some(line) = lines.peek() {
                        if line.starts_with("$") {
                            break;
                        }
                        let parts = lines.next().unwrap().split(" ").collect::<Vec<_>>();
                        if parts[0] == "dir" {
                            self.dirs.push(Box::new(Self::new(&parts[1])));
                        } else {
                            self.files.push(File {
                                name: parts[1].to_string(),
                                size: parts[0].parse().unwrap(),
                            });
                        }
                    } else {
                        break;
                    }
                }
            } else {
                let parts = contents.split(" ").collect::<Vec<_>>();
                if parts[2] == ".." {
                    break;
                } else {
                    self.dirs
                        .iter_mut()
                        .find(|d| d.name == parts[2])
                        .unwrap()
                        .build(lines);
                }
            }
        }
    }

    fn size(&mut self) -> i32 {
        if let Some(v) = self._size {
            v
        } else {
            let v = self.files.iter().map(|f| f.size).sum::<i32>()
                + self.dirs.iter_mut().map(|d| d.size()).sum::<i32>();
            self._size = Some(v);
            v
        }
    }

    fn all<'a>(&'a self) -> Vec<&'a Dir> {
        let mut result = Vec::new();
        self._all(&mut result);
        result
    }

    fn _all<'a>(&'a self, result: &mut Vec<&'a Dir>) {
        for dir in self.dirs.iter() {
            result.push(dir);
            dir._all(result);
        }
    }
}

fn part_one(lines: &[&str]) -> i32 {
    let mut dir = parse_contents(&lines);
    let _ = dir.size();
    dir.all().iter()
        .map(|d| d._size.unwrap())
        .filter(|v| *v < 100000)
        .sum()
}

fn part_two(lines: &[&str]) -> i32 {
    let mut dir = parse_contents(&lines);
    let free_space = 70000000 - dir.size();
    let mut sizes = dir.all().iter()
        .map(|d| d._size.unwrap()).collect::<Vec<_>>();
    sizes.sort();
    *sizes.iter().find(|s| *s + free_space >= 30000000).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_STR: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_one() {
        let lines = TEST_STR.lines().collect::<Vec<_>>();
        assert_eq!(95437, part_one(&lines));
    }

    #[test]
    fn test_part_one_run() {
        let content = fs::read_to_string("day_7.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(1348005, part_one(&lines));
    }

    #[test]
    fn test_part_two() {
        let lines = TEST_STR.lines().collect::<Vec<_>>();
        assert_eq!(24933642, part_two(&lines));
    }

    #[test]
    fn test_part_two_run() {
        let content = fs::read_to_string("day_7.txt").unwrap();
        let lines = content.lines().collect::<Vec<_>>();
        assert_eq!(12785886, part_two(&lines));
    }
}
