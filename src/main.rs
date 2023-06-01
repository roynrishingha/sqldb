use sqldb::{input::InputBuffer, utils};

fn main() -> std::io::Result<()> {
    let mut input_buffer = InputBuffer::new();

    utils::print_db_details();

    while input_buffer.buffer != Some(".exit".to_string()) {
        utils::print_prompt()?;
        input_buffer.read_input();

        if let Some(ref buffer) = input_buffer.buffer {
            match buffer.as_str() {
                ".exit" => break,
                _ => eprintln!("Unrecognized command {}", buffer),
            }
        }
    }

    Ok(())
}
