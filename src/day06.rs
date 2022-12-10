pub fn day06(lines: Vec<String>) -> (usize, usize) {
    let message = lines.get(0).unwrap();
    let partone = find_message_start(message, 4);
    let parttwo = find_message_start(message, 14);
    (partone, parttwo)
}

fn find_message_start(message: &str, packet_length: usize) -> usize {
    'packets: for (i, packet) in message
        .chars()
        .collect::<Vec<_>>()
        .windows(packet_length)
        .enumerate()
    {
        let mut sorted: Vec<char> = packet.into();
        sorted.sort();
        for j in 0..(packet_length - 1) {
            // if there's a repeat, try the next packet
            if sorted.get(j).unwrap() == sorted.get(j + 1).unwrap() {
                continue 'packets;
            }
        }
        return i + packet_length;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_06() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day06.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day06(lines), (1361, 3263));
    }

    #[test]
    fn test_find_message_start() {
        assert_eq!(find_message_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(find_message_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_message_start("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            find_message_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            find_message_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );

        assert_eq!(find_message_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_message_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_message_start("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            find_message_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            find_message_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
