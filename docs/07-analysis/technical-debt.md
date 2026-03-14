# Technical Debt

While the system is undergoing a significant re-architecture, several areas of technical debt remain.

## 1. Architectural Leakage
The legacy `ogl-core/src/install_detector.rs` still contains concrete platform-specific logic. This violates the Clean Architecture principle that the core should not know about infrastructure details.
- **Impact**: Harder to unit test the core without dealing with OS side-effects.
- **Risk**: Low, but increasing as more platforms are added.

## 2. Shared State Synchronization
The use of a single `Arc<Mutex<AppUiState>>` to bridge the GUI and background services can lead to lock contention and complex borrowing rules.
- **Impact**: Potential UI micro-stutters during heavy background I/O.
- **Risk**: Medium for future UI responsiveness.

## 3. Lack of Automated UI Testing
Current verification is highly manual.
- **Impact**: Regression testing is slow and error-prone.
- **Risk**: High for long-term reliability.

## 4. Minimum Documentation (Legacy)
Many legacy modules lack comprehensive doc-comments and type-level safety documentation.
- **Impact**: Higher onboarding time for new developers.
- **Risk**: Low.
