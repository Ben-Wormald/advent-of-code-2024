#[derive(Clone, Debug)]
enum Block {
    File(File),
    Free(Free),
}
impl Block {
    fn grow(&mut self, length: usize) {
        match self {
            Block::File(file) => file.length += length,
            Block::Free(free) => free.length += length,
        }
    }

    fn shrink(&mut self, length: usize) -> usize {
        match self {
            Block::File(file) => {
                file.length -= length;
                file.length
            },
            Block::Free(free) => {
                free.length -= length;
                free.length
            },
        }
    }

    fn id(&self) -> Option<usize> {
        match self {
            Block::File(File { id, .. }) => Some(*id),
            Block::Free(_) => None,
        }
    }

    fn length(&self) -> usize {
        match self {
            Block::File(File { length, .. }) => *length,
            Block::Free(Free { length }) => *length,
        }
    }
}

#[derive(Clone, Debug)]
struct File {
    length: usize,
    id: usize,
}

#[derive(Clone, Debug)]
struct Free {
    length: usize,
}

trait Disk {
    fn compact(&mut self);
    fn compact_two(&mut self);
    fn move_file(&mut self);
    fn is_compacted(&self) -> bool;
    fn get_checksum(&self) -> usize;
    fn merge_free(&mut self);
}
impl Disk for Vec<Block> {
    fn compact(&mut self) {
        loop {
            if self.is_compacted() {
                break;
            }

            self.move_file();
        }
    }

    fn compact_two(&mut self) {
        let last_id = self.iter().max_by(|block_a, block_b|
            block_a.id().unwrap_or(0).cmp(&block_b.id().unwrap_or(0))
        ).unwrap().id().unwrap();

        for id in (0..=last_id).rev() {
            if let Some((file_index, file)) = self.iter().enumerate().find(|(_i, block)|
                block.id() == Some(id)
            ) {
                let file = file.clone();

                if let Some((free_index, free)) = self.iter_mut().enumerate().find(|(_i, block)|
                    matches!(block, Block::Free(free_block) if free_block.length >= file.length())
                ) {
                    if free_index < file_index {
                        let mut file_index = file_index;

                        if free.shrink(file.length()) == 0 {
                            self.remove(free_index);
                        } else {
                            file_index += 1;
                        }

                        self.insert(free_index, file.clone());

                        self.remove(file_index);

                        self.insert(file_index, Block::Free(Free { length: file.length() }));

                        self.merge_free();
                    }
                }
            }
        }
    }

    fn move_file(&mut self) {
        let (free_index, first_free) = self.iter_mut().enumerate().find(|(_i, block)|
            matches!(block, Block::Free(_))
        ).unwrap();
        
        if first_free.shrink(1) == 0 {
            self.remove(free_index);
        }

        let (file_index, last_file) = self.iter_mut().enumerate().filter(|(_i, block)|
            matches!(block, Block::File(_))
        ).last().unwrap();
        let file_id = last_file.id();

        if last_file.shrink(1) == 0 {
           self.remove(file_index);
        }

        if free_index == 0 {
            self.insert(0, Block::File(File { length: 1, id: file_id.unwrap() }));
        } else if let Some(previous_block) = self.get_mut(free_index - 1) {
            if previous_block.id() == file_id {
                previous_block.grow(1);
            } else {
                self.insert(free_index, Block::File(File { length: 1, id: file_id.unwrap() }));
            }
        }
    }

    fn is_compacted(&self) -> bool {
        let last_file_index = self
            .iter()
            .enumerate()
            .filter(|(_i, block)| matches!(block, Block::File(_)))
            .last()
            .unwrap()
            .0;

        self
            .iter()
            .position(|block| matches!(block, Block::Free(_)))
            .is_none_or(|first_free_index| first_free_index > last_file_index)
    }

    fn get_checksum(&self) -> usize {
        self
            .iter()
            .fold((0, 0), |(sum, start_index), block| {
                match block {
                    Block::File(File { length, id }) => {
                        let mut sum = sum;
                        let end_index = start_index + length;

                        for index in start_index..end_index {
                            sum += index * id;
                        }

                        (sum, end_index)
                    },
                    Block::Free(Free { length }) => (sum, start_index + length),
                }
            })
            .0
    }

    fn merge_free(&mut self) {
        let mut index = 0;

        while index < self.len() - 1 {
            let (block_a, block_b) = (self[index].clone(), self[index + 1].clone());
            let additional_length = match (block_a, block_b) {
                (Block::Free(_), Block::Free(Free { length })) => Some(length),
                _ => None,
            };

            if let Some(length) = additional_length {
                self[index].grow(length);
                self.remove(index + 1);
            } else {
                index += 1;
            }
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let mut disk = get_disk(input);
    disk.compact();
    disk.get_checksum()
}

pub fn solve_part_two(input: &str) -> usize {
    let mut disk = get_disk(input);
    disk.compact_two();
    disk.get_checksum()
}

fn get_disk(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();

    for (index, length) in input.trim().char_indices() {
        let length = length.to_string().parse().unwrap();

        if length == 0 {
            continue;
        }

        if index % 2 == 0 {
            let id = index / 2;
            blocks.push(Block::File(File { length, id }));
        } else {
            blocks.push(Block::Free(Free { length }));
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402\n";

    #[test]
    fn part_one() {
        let expected = 1928;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 2858;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
