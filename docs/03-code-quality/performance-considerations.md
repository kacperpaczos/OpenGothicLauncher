# Performance Considerations

## Startup Performance
- **Lazy Loading**: The GUI starts immediately; expensive operations like registry scanning for multiple games are performed asynchronously in background tasks.
- **Binary Size**: Using a workspace and shared dependencies keeps the total binary size relatively low for a Rust application.

## Runtime Efficiency
- **Non-blocking I/O**: The use of `Tokio` ensures that network downloads and filesystem scans do not block the UI thread (GTK event loop).
- **Polling Frequency**: The UI refresh rate is set to 500ms, which is a balanced trade-off between responsiveness and CPU usage.

## Resource Usage
- **Memory**: GTK4 and Tokio have a base memory footprint, but the application logic itself is very lean, primarily holding metadata strings and some configuration state.
- **Disk I/O**: Brute-force scanning is the most IO-intensive task; it is designed to be throttled or limited in scope to avoid disk thrashing.
