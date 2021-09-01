pub fn display_output<T>(output: T) -> std::io::Result<()>
where
    T: serde::Serialize,
{
    use pangalactic_errorutil::debug_to_std_io_error;

    serde_json::to_writer_pretty(std::io::stdout(), &output).map_err(debug_to_std_io_error)
}
