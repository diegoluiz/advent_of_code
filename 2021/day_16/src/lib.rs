use helper::helper;

fn hex_to_bin(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };

    b.to_string()
}

fn bin_to_dec(b: &str) -> usize {
    usize::from_str_radix(b, 2).unwrap()
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Message {
    literal: String,
    version: usize,
    type_id: usize,

    value: usize,
    value_message: Option<ValueMessage>,
    operator_message: Option<OperatorMessage>,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]

struct ValueMessage {
    value: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct OperatorMessage {
    length_type_id: usize,
    messages: Vec<Message>,
    value: usize,
}

fn process_package(idx: &mut usize, commands: &String, messages: &mut Vec<Message>) {
    let packet_ini = *idx;
    let version = bin_to_dec(&commands[*idx..*idx + 3]);
    *idx += 3;
    let type_id = bin_to_dec(&commands[*idx..*idx + 3]);
    *idx += 3;
    if type_id == 4 {
        let mut s = String::new();
        loop {
            let block = &commands[*idx..*idx + 5];
            let has_next = &block[0..1];
            s.push_str(&block[1..]);

            *idx += 5;
            if has_next == "0" {
                break;
            }
        }
        let value = bin_to_dec(s.as_str());

        let value_message = ValueMessage { value: value };

        let message = Message {
            literal: commands[packet_ini..*idx].to_string(),
            version: version,
            type_id: type_id,
            value_message: Some(value_message),
            value,
            operator_message: None,
        };
        messages.push(message);
    } else {
        let length_type_id = bin_to_dec(&commands[*idx..*idx + 1]);
        *idx += 1;
        let mut sub_messages = Vec::<Message>::new();

        match length_type_id {
            0 => {
                let sub_packets_size = bin_to_dec(&commands[*idx..*idx + 15]);
                *idx += 15;
                let sub_packets_end = *idx + sub_packets_size;
                while *idx < sub_packets_end {
                    process_package(idx, &commands, &mut sub_messages);
                }
            }
            1 => {
                let sub_packets_count = bin_to_dec(&commands[*idx..*idx + 11]);
                *idx += 11;
                for _ in 0..sub_packets_count {
                    process_package(idx, &commands, &mut sub_messages);
                }
            }
            _ => {}
        }

        let values = sub_messages.iter().map(|x| x.value).collect::<Vec<_>>();

        let value = match type_id {
            0 => values.iter().fold(0, |acc, curr| acc + curr),
            1 => values.iter().fold(1, |acc, curr| acc * curr),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => {
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        };

        let op_message = OperatorMessage {
            length_type_id,
            messages: sub_messages,
            value,
        };

        let message = Message {
            literal: commands[packet_ini..*idx].to_string(),
            version: version,
            type_id: type_id,
            value_message: None,
            operator_message: Some(op_message),
            value,
        };

        messages.push(message);
    }
}

fn sum_version(message: &Message) -> usize {
    if message.operator_message.is_some() {
        let sums: usize = message
            .operator_message
            .clone()
            .unwrap()
            .messages
            .iter()
            .map(|x| sum_version(x))
            .sum();
        return sums + message.version;
    }
    if message.value_message.is_some() {
        return message.version;
    }
    0
}

pub struct Challenge {}
pub struct ChallengeInput {
    data: String,
}

impl helper::Process<ChallengeInput> for Challenge {
    fn challenge_name(&self) -> String {
        "day_16".to_string()
    }

    fn load_input(&self, base_dir: String, is_sample_input: bool) -> ChallengeInput {
        let input_file_path = helper::get_input_file(base_dir, is_sample_input);
        let input = helper::load_input(input_file_path);
        ChallengeInput { data: input }
    }

    fn part_01(&self, input: &ChallengeInput) -> String {
        let input = input.data.clone();
        let mut commands = String::new();

        for c in input.chars() {
            let x = hex_to_bin(c);
            commands.push_str(x.as_str());
        }

        let mut idx = 0 as usize;
        let mut messages = Vec::<Message>::new();
        loop {
            process_package(&mut idx, &commands, &mut messages);
            if idx + 8 > commands.len() {
                break;
            }
        }

        let sum: usize = messages.iter().map(|x| sum_version(x)).sum();
        sum.to_string()
    }

    fn part_02(&self, input: &ChallengeInput) -> String {
        let input = input.data.clone();
        let mut commands = String::new();

        for c in input.chars() {
            let x = hex_to_bin(c);
            commands.push_str(x.as_str());
        }

        let mut idx = 0 as usize;
        let mut messages = Vec::<Message>::new();
        loop {
            process_package(&mut idx, &commands, &mut messages);
            if idx + 8 > commands.len() {
                break;
            }
        }

        messages[0].value.to_string()
    }
}
