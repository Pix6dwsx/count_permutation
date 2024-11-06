use rand::Rng;

// Функция для подсчета минимального количества перемещений, чтобы уравнять груз на всех кораблях
fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let total_weight: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    // Проверка, возможно ли распределить груз поровну
    if total_weight % n != 0 {
        return -1;
    }

    let target_weight = total_weight / n;
    let mut moves = 0;
    let mut balance = 0;

    for &weight in shipments {
        balance += weight as i32 - target_weight as i32;
        moves += balance.abs();
    }

    moves as i32
}

// Функция генерации вектора весов, который можно распределить поровну
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let target_weight = rng.gen_range(10..100);
    let mut shipments = vec![target_weight; n];

    // Делаем небольшую корректировку, чтобы общая сумма оставалась кратной n
    let adjustment = rng.gen_range(0..n);
    for i in 0..adjustment {
        shipments[i] += 1;
    }

    shipments
}

// Пример использования функций
fn main() {
    // Пример с фиксированным вектором
    let shipments = vec![1, 1, 1, 1, 6];
    println!("Shipments: {:?}", shipments);
    println!("Minimum moves: {}", count_permutation(&shipments));

    // Пример сгенерированного вектора
    let generated_shipments = gen_shipments(5);
    println!("Generated shipments: {:?}", generated_shipments);
    println!("Minimum moves for generated shipments: {}", count_permutation(&generated_shipments));
}
