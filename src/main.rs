use tokio;

#[tokio::main]
async fn main(){
    println!("Aync task starting");
    let task1 = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("task 1 complted");
    });

    let task2 = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("task 2 complted");
    });

    task1.await.unwrap();
    task2.await.unwrap();

    println!("all tasks completed!")
    

}

// Practical Example: Building a Simple Web Server with Concurrency
