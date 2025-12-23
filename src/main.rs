mod extractor;


fn main() {
    println!(">> running DuelingBook extractor...");

    let deck = extractor::fetch::deck("https://www.duelingbook.com/deck?id=18239213");

    println!("deck = {:?}", deck);
}
