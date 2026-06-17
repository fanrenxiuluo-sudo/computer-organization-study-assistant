import { Component, type ReactNode } from "react";

type Props = { children: ReactNode };
type State = { hasError: boolean; error: Error | null };

export class ErrorBoundary extends Component<Props, State> {
  state: State = { hasError: false, error: null };

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo): void {
    console.error("[ErrorBoundary] Uncaught error:", error, errorInfo);
  }

  handleReset = () => {
    this.setState({ hasError: false, error: null });
  };

  render() {
    if (this.state.hasError) {
      return (
        <div style={{ padding: 48, textAlign: "center" }}>
          <h2>页面出错了</h2>
          <p>{this.state.error?.message ?? "未知错误"}</p>
          <button
            onClick={this.handleReset}
            style={{ marginTop: 16, padding: "8px 24px", cursor: "pointer" }}
          >
            重试
          </button>
          <p style={{ marginTop: 8, color: "#666" }}>或刷新页面。</p>
        </div>
      );
    }
    return this.props.children;
  }
}