use std::fs::read_to_string;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Pulse{
    High,
    Low
}

enum Module{
    FlipFlop,
    Conjunction,
    Broadcast,
    Button
}


trait RecieveSignal{
    fn recieve(&mut self, pulse_in: &Pulse, sender: &str)->Option<Vec<(String,Pulse)>> ;
}

struct FlipFlopModule{
    name:String,
    destinations: Vec<String>,
    current_pulse: Pulse,
}
impl RecieveSignal for FlipFlopModule{
    fn recieve(&mut self, pulse_in: &Pulse, sender: &str)->Option<Vec<(String,Pulse)>> {
        let output_pulse = match (&self.current_pulse, pulse_in){
            (Pulse::Low,  Pulse::Low ) => Some(Pulse::High),
            (Pulse::High, Pulse::Low ) => Some(Pulse::Low),
            _ => None
        };
        match output_pulse{
            None => None,
            Some(pulse) => {
                Some(self.destinations.iter().map(|dest| (dest.clone(), pulse)).collect::<Vec<(String,Pulse)>>())
            }
        }
    }
}

struct Conjuction{
    name:String,
    destinations: Vec<String>,
    input_connections: Vec<String>,
    memory: Vec<Pulse>,
}
impl RecieveSignal for Conjuction{
    fn recieve(&mut self, pulse_in: &Pulse, sender:&str)->Option<Vec<(String,Pulse)>>  {
        let mut found_sender_in_inputs = false;
        for (idx, seen_connection) in self.input_connections.iter().enumerate(){
            if seen_connection.as_str() == sender{
                self.memory[idx] = *pulse_in;
                found_sender_in_inputs = true;
                break;
            }
        }
        // Note: If there is a connection that was never activated, this will not take it into account until it IS activated.
        // Might cause issues. Testing will tell...
        if !found_sender_in_inputs{
            self.input_connections.push(sender.to_string());
            self.memory.push(Pulse::High)
        }

        let output_pulse = self.memory.iter().fold(Pulse::Low, |acc,pulse|{
            if *pulse == Pulse::High{
                acc
            }else{
                Pulse::High
            }
        });
        Some(self.destinations.iter().map(|dest| (dest.clone(), output_pulse)).collect::<Vec<(String,Pulse)>>())
    }
}

struct Broadcast{
    name:String,
    destinations: Vec<String>,
}
impl RecieveSignal for Broadcast{
    fn recieve(&mut self, pulse_in: &Pulse, sender: &str)->Option<Vec<(String,Pulse)>>  {
        Some(self.destinations.iter().map(|dest| (dest.clone(), *pulse_in)).collect::<Vec<(String,Pulse)>>())

    }
}

struct Button{
    name:String,
    destinations: Vec<String>,
}
impl RecieveSignal for Button{
    fn recieve(&mut self, pulse_in: &Pulse, sender: &str)->Option<Vec<(String,Pulse)>>  {
        Some(self.destinations.iter().map(|dest| (dest.clone(), Pulse::Low)).collect::<Vec<(String,Pulse)>>())
    }
}



pub fn main_1(file_name:&str)->Option<i32>{
    const n_button_presses: u32 = 1000;
    let data_string = read_to_string(file_name).unwrap();
    None

}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn my_test(){

    }

}
