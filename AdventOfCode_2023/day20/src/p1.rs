use std::{collections::{HashMap, HashSet, VecDeque}, fmt::{Debug, Display}, fs::read_to_string, hash::Hash};

#[derive(Debug,Clone,Copy,PartialEq)]
enum Pulse{
    High,
    Low
}
impl Display for Pulse{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            Pulse::High => write!(f,"high"),
            Pulse::Low => write!(f,"low")
        }
    }
}
enum Module{
    FlipFlop(FlipFlopModule),
    Conjunction(ConjuctionModule),
    Broadcast(BroadcastModule),
    Sink(SinkModule),
}
impl Module{
    fn get_name(&self)->&str{
        match self{
            Module::FlipFlop(module)      => &module.name,
            Module::Conjunction(module) => &module.name,
            Module::Broadcast(module)    => &module.name,
            Module::Sink(module)              => &module.name,
        }
    }
    fn get_destinations(&self) -> &Vec<String>{
        match self{
            Module::FlipFlop(module)      => &module.destinations,
            Module::Conjunction(module) => &module.destinations,
            Module::Broadcast(module)    => &module.destinations,
            Module::Sink(module)              => &module.destinations,
        }
    }
    fn get_counts(&self) -> [u128;2]{
        match self{
            Module::FlipFlop(module)      => [module.low_count, module.high_count],
            Module::Conjunction(module) => [module.low_count, module.high_count],
            Module::Broadcast(module)    => [module.low_count, module.high_count],
            Module::Sink(module)              => [module.low_count, module.high_count],
        }
    }
}
impl From<[&str;2]> for Module{
    fn from([module_str,dest_str]: [&str;2]) -> Self{
        let mut all_chars = module_str.chars();
        let destinations = dest_str.split(", ").map(|s| s.to_string()).collect::<Vec<String>>();
        let first_char = all_chars.next().unwrap();
        let module_name = all_chars.collect::<String>();
        match first_char{
            'b' => Self::Broadcast(BroadcastModule::new(destinations)),
            '%' => Self::FlipFlop(FlipFlopModule::new(module_name,destinations)),
            '&' => Self::Conjunction(ConjuctionModule::new(module_name,destinations)),
            _   => Self::Sink(SinkModule::new())
        }        
    }

}
impl From<&str> for Module{
    fn from(s_in: &str) -> Self{
        let [module_str, dest_str]: [&str;2] = s_in.split(" -> ").collect::<Vec<&str>>().try_into().unwrap();
        Self::from([module_str, dest_str])
    }
}

struct SinkModule{
    name: String,
    destinations: Vec<String>,
    low_count:u128,
    high_count:u128,
}
impl SinkModule{
    fn new()->Self{
        Self{name:"".to_string(), destinations:vec![], low_count:0, high_count:0}
    }
}

struct FlipFlopModule{
    name: String,
    destinations: Vec<String>,
    low_count:u128,
    high_count:u128,
    is_on:bool

}
impl FlipFlopModule{
    fn new(name:String,destinations:Vec<String>)->Self{
        Self{name,destinations, is_on: false, low_count:0,high_count:0}
    }
}

struct ConjuctionModule{
    name: String,
    destinations: Vec<String>,
    input_connections: Vec<String>,
    memory: Vec<Pulse>,
    low_count:u128,
    high_count:u128,

}
impl ConjuctionModule{
    fn new(name:String,destinations: Vec<String>)->Self{
        Self{name, destinations, input_connections:vec![], memory:vec![],low_count:0,high_count:0}
    }
}
impl Debug for ConjuctionModule{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConjuctionModule")
            .field("name", &self.name)
            .field("destinations", &self.destinations)
            .field("input_connections", &self.input_connections)
            .field("memory", &self.memory)
            .field("low_count", &self.low_count)
            .field("high_count", &self.high_count)
            .finish()
    }
}
struct BroadcastModule{
    name: String,
    destinations: Vec<String>,
    low_count:u128,
    high_count:u128,

}
impl BroadcastModule {
    fn new(destinations: Vec<String>) -> Self{
        Self{name: "broadcast".to_string(),destinations,low_count:0,high_count:0}
    }
}
trait RecieveSignal{
    fn recieve(&mut self, pulse_in: &Pulse, sender: &str)->Option<Vec<(String,Pulse)>> ;
}
impl RecieveSignal for Module{
    fn recieve(&mut self, pulse_in: &Pulse, sender: &str)->Option<Vec<(String,Pulse)>>{
        match self{
            Module::FlipFlop(module)      => module.recieve(pulse_in, sender),
            Module::Conjunction(module) => module.recieve(pulse_in, sender),
            Module::Broadcast(module)    => module.recieve(pulse_in, sender),
            Module::Sink(module)    => module.recieve(pulse_in, sender),
        }
    }
}
impl RecieveSignal for FlipFlopModule{
    fn recieve(&mut self, pulse_in: &Pulse, _sender: &str)->Option<Vec<(String,Pulse)>>{
        match pulse_in{
            Pulse::High => self.high_count += 1,
            Pulse::Low => self.low_count +=1,
        }
        if pulse_in == &Pulse::High{
            return None;
        }
        let pulse_out = match self.is_on {
            true => Pulse::Low,
            false => Pulse::High
        };
        self.is_on = !self.is_on; // Invert current on/off state.
        let output = self.destinations.iter().map(|dest| (dest.clone(),pulse_out) ).collect::<Vec<(String,Pulse)>>();
        Some(output)
    }
}
impl RecieveSignal for ConjuctionModule{
    fn recieve(&mut self, pulse_in: &Pulse, sender: &str)->Option<Vec<(String,Pulse)>>{
        #[cfg(none)]{
            println!();
            println!("______ {pulse_in} ______");
            println!("{self:#?}");
            println!();
        }

        match pulse_in{
            Pulse::High => self.high_count += 1,
            Pulse::Low => self.low_count +=1,
        }
        

        // Update the memory for the sender.
        for (connection, memory) in self.input_connections.iter_mut().zip(self.memory.iter_mut()){
            if connection == sender{
                *memory = *pulse_in;
                break;
            }
        }

        let all_high = self.memory
            .iter()
            .all(|pulse| pulse == &Pulse::High );

        let output_pulse = match all_high{
            true => Pulse::Low,
            false => Pulse::High
        };        
            #[cfg(none)]{
                let name = &self.name;
                println!("{name} recieved {pulse_in} from {sender}.");
                println!("Now memory is:");
                println!();
            }
        Some(self.destinations.iter().map(|dest| (dest.clone(),output_pulse)).collect::<Vec<(String,Pulse)>>())
        
    }
}
impl RecieveSignal for BroadcastModule{
    fn recieve(&mut self, pulse_in: &Pulse, _sender: &str)->Option<Vec<(String,Pulse)>>{
        match pulse_in{
            Pulse::High => self.high_count += 1,
            Pulse::Low => self.low_count +=1,
        }
        Some( self.destinations
                .iter()
                .map(|dest|(dest.clone(), *pulse_in))
                .collect::<Vec<(String,Pulse)>>()
            )
    }
}
impl RecieveSignal for SinkModule{
    fn recieve(&mut self, pulse_in: &Pulse, _sender: &str)->Option<Vec<(String,Pulse)>>{
        match pulse_in{
            Pulse::High => self.high_count += 1,
            Pulse::Low => self.low_count +=1,
        }
        None
    }
}
fn get_all_conjunctions(all_modules: &HashMap<String,Module>)->HashMap<String,Vec<String>>{
    let all_conjunctions = all_modules
        .iter()
        .filter_map(|(name, module)|{
            match module{
                Module::Conjunction(_) => Some((name.clone(),vec![])),
                _ => None
            }
        }).collect::<HashMap<String,Vec<String>>>();
    
    all_conjunctions

}
fn get_conjunction_connections(mut all_conjunctions: HashMap<String,Vec<String>>, all_modules: &HashMap<String,Module>)->HashMap<String,Vec<String>>{
    for (parent_name, module) in all_modules.iter(){
        let desinations = module.get_destinations();
        let connected_conjunctions = desinations
            .iter()
            .filter_map(|name| {
                if all_conjunctions.contains_key(name){
                    Some(name.as_str())
                }else{
                    None
                }
            }).collect::<Vec<&str>>();
        for connected_conjunction in connected_conjunctions.iter(){
            if let Some(conjunction_parents) = all_conjunctions.get_mut(*connected_conjunction){
                conjunction_parents.push(parent_name.clone())
            }
        }
    }
    all_conjunctions
}
fn insert_parents_into_memory(all_conjunctions: HashMap<String,Vec<String>>, mut all_modules: HashMap<String,Module>)->HashMap<String,Module>{
    for (key,val) in all_conjunctions.into_iter(){
        if let Some(module) = all_modules.get_mut(&key){
            match module{
                Module::Conjunction(conjunction_module) => {
                    conjunction_module.input_connections = val;
                    conjunction_module.memory = vec![Pulse::Low; conjunction_module.input_connections.len()]
                },
                _ => ()
            }
        }
    }
    all_modules
}
fn connect_conjunction_modules_to_parents(mut all_modules: HashMap<String,Module>)->HashMap<String,Module>{
    // Goes through all modules and find the ones that are conjunctions. 
    let mut all_conjunctions = get_all_conjunctions(&all_modules);

    // Then goes through all modules and log the ones that link to conjunctions to that corresponding conjunction.
    all_conjunctions = get_conjunction_connections(all_conjunctions,&all_modules);

    // Insert the connected parents (and the corresponding memory) into the module map.
    all_modules = insert_parents_into_memory(all_conjunctions, all_modules);

    // Return the module map back out.
    all_modules
}
fn add_sinks(mut all_modules: HashMap<String,Module>)->HashMap<String,Module>{
    let all_destinations = all_modules.values().flat_map(|module| module.get_destinations().clone() ).collect::<HashSet<String>>();
    for destination in all_destinations.into_iter(){
        if all_modules.get(&destination).is_some(){
            continue;
        }
        all_modules.insert(destination, Module::Sink(SinkModule::new()));
    }
    all_modules
}
fn extract_modules_from_string(data_string:String)->HashMap<String,Module>{
    let mut all_modules = data_string
        .lines()
        .map(|line|{
            let module = Module::from(line);
            (module.get_name().to_string(), module)
        }).collect::<HashMap<String,Module>>();

    all_modules = connect_conjunction_modules_to_parents(all_modules);
    all_modules = add_sinks(all_modules);
    all_modules
}

fn recursive_signal_completion(mut module_dict: HashMap<String,Module>,current_name:&str, sender: &str, pulse_in: Pulse)->HashMap<String,Module>{
    // Depth first signal sender.
    let current = match module_dict.get_mut(current_name){
        None => {
            println!("Failed to get >{current_name}< from the module_dict...");
            module_dict.keys().for_each(|line| println!(">{line}<"));
            panic!();
        },
        Some(v) => v,
    };
    let transmission = current.recieve(&pulse_in, sender);
    if transmission.is_none(){
        return module_dict;
    }
    for (target,pulse) in transmission.unwrap().into_iter(){
        #[cfg(test)]
        println!("{current_name} -{pulse}-> {target}");
        module_dict = recursive_signal_completion(module_dict, target.as_str(), current_name, pulse)
    }
    module_dict
}

fn width_first_signal_completion(mut module_dict: HashMap<String,Module>,current_name:&str, sender: &str, pulse_in: Pulse)->HashMap<String,Module>{
    let mut queue = VecDeque::new();
    queue.push_back((current_name.to_string(),pulse_in, sender.to_string()));

    while let Some((name, pulse,sender)) = queue.pop_front() {
        #[cfg(test)]
        println!("{sender} -{pulse}-> {name}");
        let module = match module_dict.get_mut(&name){
            Some(v) => v,
            None => continue
        };
        if let Some(transmission)  = module.recieve(&pulse, &sender){
            for (new_name, new_pulse) in transmission.into_iter(){
                queue.push_back( (new_name,new_pulse,name.clone()) )
            }
        }
    }
    module_dict
}

pub fn main_1(file_name:&str)->Option<u128>{
    const N_BUTTON_PRESSES:u16 = 1000;
    let data_string = read_to_string(file_name).unwrap();
    let mut module_dict = extract_modules_from_string(data_string);
    for i in 0..N_BUTTON_PRESSES{
        // println!("Press nr. {i}");
        module_dict = width_first_signal_completion(module_dict, "broadcast","button",Pulse::Low);
    }
    let [low_count, high_count] = module_dict
        .values()
        .into_iter()
        .fold([0,0], |acc,module| {
            let counts = module.get_counts();
            [acc[0] + counts[0], acc[1]+counts[1]]
        } );
    #[cfg(test)]{
        if file_name ==r"src\dummy.txt"{
            println!("high: {high_count}, low: {low_count}");
            assert_eq!(low_count,8000);
            assert_eq!(high_count,4000);
        }else if file_name ==r"src\dummy2.txt"{
            println!("high: {high_count}, low: {low_count}");
            assert_eq!(low_count,4250);
            assert_eq!(high_count,2750);
        }
    }
    Some(low_count*high_count)
}

#[cfg(test)]
    mod tests{
    use super::*;

    #[test]
    fn all_destinations_must_exist(){
        let data_string = read_to_string(r"src\dummy.txt").unwrap();
        let module_dict = extract_modules_from_string(data_string);
        for (_,module) in module_dict.iter(){
            for destination in module.get_destinations(){
                assert!(module_dict.get(destination).is_some(),"Missing name: >>>{destination}<<<");
            }
        }
    }

    #[test]
    fn count_test1(){
        let data_string = read_to_string(r"src\dummy.txt").unwrap();
        let mut module_dict = extract_modules_from_string(data_string);
        module_dict = width_first_signal_completion(module_dict, "broadcast","button",Pulse::Low);
        let [low_count, high_count] = module_dict
            .values()
            .into_iter()
            .fold([0,0], |acc,module| {
                let counts = module.get_counts();
                [acc[0] + counts[0], acc[1]+counts[1]]
            });
        println!("high: {high_count}, low: {low_count}");
        assert_eq!([low_count, high_count], [8,4])
    }

    #[test]
    fn count_test2(){
        let data_string = read_to_string(r"src\dummy2.txt").unwrap();
        let mut module_dict = extract_modules_from_string(data_string);
        module_dict = width_first_signal_completion(module_dict, "broadcast","button",Pulse::Low);
        let [low_count, high_count] = module_dict
            .values()
            .into_iter()
            .fold([0,0], |acc,module| {
                let counts = module.get_counts();
                [acc[0] + counts[0], acc[1]+counts[1]]
            });
        println!("high: {high_count}, low: {low_count}");
        assert_eq!([low_count, high_count], [4,4])
    }

    #[test]
    fn conjunction_all_combinations_test(){
        let name = "dummy".to_string();
        let destinations = vec!["a".to_string(),"b".to_string()];
        let mut module = ConjuctionModule::new(name, destinations); 
        module.input_connections = vec!["x".to_string(), "y".to_string()];
        
        // If we get a high input leading to all-high memory, we get low. Otherwise we get high.
        for a_memory in [Pulse::Low, Pulse::High].iter() {
            for b_memory in [Pulse::Low, Pulse::High].iter() {
                module.memory = vec![*a_memory, *b_memory];
                let output = module.recieve(&Pulse::High, "y");

                let expected = {
                    match (a_memory, b_memory) {
                        (Pulse::High, Pulse::High) => Pulse::Low,
                        (Pulse::High, Pulse::Low) => Pulse::Low,
                        (Pulse::Low, Pulse::High) => Pulse::High,
                        (Pulse::Low, Pulse::Low) => Pulse::High,
                    }
                };
                assert_eq!(expected, output.unwrap()[0].1,"Did not get the output we expected for this high input...");
            }
        }

        // If we get a high input leading to all-high memory, we get low. Otherwise we get high.
        for a_memory in [Pulse::Low, Pulse::High].iter() {
            for b_memory in [Pulse::Low, Pulse::High].iter() {
                module.memory = vec![*a_memory, *b_memory];
                let output = module.recieve(&Pulse::High, "x");
                let expected = {
                    match (a_memory, b_memory) {
                        (Pulse::High, Pulse::High) => Pulse::Low,
                        (Pulse::High, Pulse::Low) => Pulse::High,
                        (Pulse::Low, Pulse::High) => Pulse::Low,
                        (Pulse::Low, Pulse::Low) => Pulse::High,
                    }
                };
                assert_eq!(expected, output.unwrap()[0].1,"Did not get the output we expected...");
            }
        }
        
        // Checking that given a low "y"-input, we allways get a high output.
        for a_memory in [Pulse::Low, Pulse::High].iter() {
            for b_memory in [Pulse::Low, Pulse::High].iter() {
                module.memory = vec![*a_memory, *b_memory];
                let output = module.recieve(&Pulse::Low, "y");
                let expected = Pulse::High; // Low input will allways yield a "high" output.
                assert_eq!(expected, output.unwrap()[0].1,"Did not get the output we expected for this high input...");
            }
        }
        
        // Checking that given a low "-input, we allways get a high output.
        for a_memory in [Pulse::Low, Pulse::High].iter() {
            for b_memory in [Pulse::Low, Pulse::High].iter() {
                module.memory = vec![*a_memory, *b_memory];
                let output = module.recieve(&Pulse::Low, "x");
                let expected = Pulse::High; // Low input will allways yield a "high" output.
                assert_eq!(expected, output.unwrap()[0].1,"Did not get the output we expected for this high input...");
            }
        }
    }


}
