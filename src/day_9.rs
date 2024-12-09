use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day9;

impl Solution for Day9 {
    fn part_1() -> String {
        let data = load_input(9);
        let nums: Vec<usize> = data
            .strip_suffix("\n")
            .unwrap_or(&data)
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let mut fs: Vec<Option<usize>> = Vec::new();
        let mut file: bool = true;
        let mut id: usize = 0;
        for num in nums {
            if file {
                for _ in 0..num {
                    fs.push(Some(id));
                }
                id += 1;
                file = false;
            } else {
                for _ in 0..num {
                    fs.push(None);
                }
                file = true;
            }
        }

        let mut compacted: Vec<usize> = Vec::new();
        let mut back = fs.clone();
        let target = fs.iter().filter(|n| n.is_some()).count();
        for num in fs.iter() {
            if compacted.len() == target {
                break;
            }
            match num {
                Some(n) => compacted.push(*n),
                None => {
                    let mut r = back.pop().unwrap();
                    while r.is_none() {
                        r = back.pop().unwrap()
                    }
                    compacted.push(r.unwrap());
                }
            }
        }

        compacted
            .iter()
            .enumerate()
            .map(|(i, n)| i * n)
            .sum::<usize>()
            .to_string()
    }

    fn part_2() -> String {
        let data = load_input(9);
        let mut nums: Vec<usize> = data
            .strip_suffix("\n")
            .unwrap_or(&data)
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        nums.push(0);
        // (id, file_len, following empty)
        let pairs: Vec<Data> = nums
            .windows(2)
            .step_by(2)
            .enumerate()
            .map(|(id, ns)| Data {
                id,
                size: ns[0],
                blank: ns[1],
            })
            .collect();

        let mut compressed = pairs.clone();
        // println!("{:?}", compressed);
        for data in pairs[1..].iter().rev() {
            let c_index = compressed
                    .iter()
                    .position(|current_data| current_data.id == data.id)
                    .unwrap();
            let slot: Option<usize> = compressed[0..c_index].iter().position(|d| d.blank >= data.size);
            // print!("{:?}", data);
            if let Some(index) = slot {
                compressed[c_index - 1].blank += data.size + compressed[c_index].blank;
                let new_rest = compressed[index].blank - data.size;
                compressed[index].blank = 0;
                let mut to_move = compressed.remove(c_index);
                to_move.blank = new_rest;
                compressed.insert(index + 1, to_move)
            }
            
            // println!("{:?}", compressed);
        }

        let mut final_vec: Vec<Option<usize>> = Vec::new();
        for num in compressed.iter() {
            for _ in 0..num.size {
                final_vec.push(Some(num.id));
            }
            for _ in 0..num.blank {
                final_vec.push(None);
            }
        }

        final_vec
            .iter()
            .enumerate()
            .filter(|(_, id)| id.is_some())
            .map(|(i, id)| i * id.unwrap())
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Clone, Copy, Debug)]
struct Data {
    id: usize,
    size: usize,
    blank: usize,
}

// struct CustomIterator<'a> {
//     /// Forwards index
//     fi: usize,
//     /// Backwards index
//     bi: usize,
//     /// Data
//     data: &'a Vec<usize>,
//     final_id: usize,
//     mode: bool,
//     forward: usize,
//     fid: usize,
//     backward: usize,
//     bid: usize,
//     fc: usize,
//     bc: usize,
// }

// impl<'a> Iterator for CustomIterator<'a> {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         while self.fc < self.bc {
//             if self.mode {
//                 self.fc += 1;
//                 // Forwards mode
//                 if self.forward > 0 {
//                     self.forward -= 1;
//                     return Some(self.fid);
//                 } else {
//                     self.fid += 1;
//                     self.forward = self.data[self.fid];
//                     self.mode = false;
//                 }
//             } else {
//                 self.bc -= 1;
//                 // Backwards mode
//                 if self.forward > 0 {
//                     if self.backward == 0 {

//                     }
//                     return Some(self.bid);
//                 } else {

//                 }
//             }
//         }

//         None
//     }
// }
