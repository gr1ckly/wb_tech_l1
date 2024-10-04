/*
Реализация паттерна "Адаптер" на примере записи о подключении сканера в память через контроллер прямого доступа к памяти
*/

// трейт - получатель
pub trait Receiver<T>{
    fn get_data(&mut self, data: T);
}

//Память
pub struct Memory{
    pub name: String,
    pub capacity_bytes: usize
}

impl Memory{
    pub fn new(name: String, capacity: usize) -> Self{
        Memory {
            name: name,
            capacity_bytes: capacity
        }
    }
}

//Запись данных в память возможна только в байтах
impl Receiver<Vec<u8>> for Memory{
    fn get_data(&mut self, data : Vec<u8>){
        self.capacity_bytes -= data.len();
        println!("Free memory: {} bytes", self.capacity_bytes);
        println!("Put bytes into memory");
        for byte in data{
            println!("Put: {}", byte)
        }
    }
}

//Контроллер прямого доступа к памяти, выступает в роли адаптера между внешним устройством и памятью
pub struct DirectMemoryAccessController{
    pub name: String,
    memory: Memory
}

impl DirectMemoryAccessController{
    pub fn new(name: String, mem: Memory) -> Self{
        DirectMemoryAccessController{
            name: name,
            memory: mem
        }
    }
}

//Транслирует сообщение от внешнего устройства в байты для записи в память
impl Receiver<String> for DirectMemoryAccessController{
    fn get_data(&mut self, data: String) {
        self.memory.get_data(data.into_bytes());
    }
}

pub struct Scanner{
    pub name:String
}

impl Scanner{
    pub fn new(name: String) -> Self{
        Scanner{
            name:name
        }
    }
    //Инициализация подключение - запись в память названия внешнего устройства
    pub fn init_connection(&mut self, mut receiver: impl Receiver<String>){
        receiver.get_data(self.name.clone());
    }
}

pub fn sim_connection_scanner(){
    let mut mem = Memory::new(String::from("Pretty memory"), 10245);
    let mut controller = DirectMemoryAccessController::new(String::from("Control"), mem);
    let mut scanner = Scanner::new(String::from("Printer Scanner 200"));
    scanner.init_connection(controller);
}