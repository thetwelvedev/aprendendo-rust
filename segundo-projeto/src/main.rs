const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main() {
    let mut total = 30;// Se não usar o mut depois de let, não vai ser alterável pois é padrão do rust
    println!("Trabalhou {} horas", total);
    
    //Usando as constantes
    let total_em_segundos = total * SECONDS_IN_HOUR;
    println!("Trabalhou {} segundos", total_em_segundos);

    total = 40; //Tem a tipagem forte
    {//Escopo interno 
        let total = 35; //Nesse novo escopo não afeta a variável externa mesmo usando o mesmo nome, ela ainda pode acessar valores do escopo externo
        println!("Trabalhou {} horas", total);
    }
    println!("Trabalhou {} horas", total);
    let total = "quarenta"; //Varieble Shadowing - Basicamente trata como objetos diferente, e a partir do ponto da nova variável com mesmo nome ela assume a partir dali somente sua nova atribuição
    println!("Trabalhou {} horas", total);
}
