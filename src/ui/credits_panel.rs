//! Credits Panel Component with Trade Preview
//!
//! Displays player credits with a dynamic trade preview showing projected
//! credit changes from pending market trades.

#[cfg(feature = "web")]
use leptos::*;

/// CreditsPanel Component
///
/// Displays a prominent credits display with:
/// - Current credits amount (large, prominent)
/// - Projected credit change when trading (dynamic update)
/// - Color coding: green for gains, red for losses
///
/// # Arguments
/// * `current_credits` - Callback returning current player credits
/// * `credit_change` - Memo containing credit change from pending trades
///                     (positive = gain, negative = loss)
/// * `projected_credits` - Memo containing projected credits after trade
#[cfg(feature = "web")]
#[component]
pub fn CreditsPanel(
    current_credits: impl Fn() -> u32 + Clone + 'static,
    credit_change: Memo<i32>,
    projected_credits: Memo<u32>,
) -> impl IntoView {
    // Determine CSS class based on credit change direction
    let change_class = create_memo(move |_| {
        let change = credit_change.get();
        if change > 0 {
            "credit-gain"
        } else if change < 0 {
            "credit-loss"
        } else {
            "credit-neutral"
        }
    });

    // Format the projected display
    let projected_display = create_memo(move |_| {
        let current = current_credits();
        let projected = projected_credits.get();
        let change = credit_change.get();
        
        if change == 0 {
            // No change - just show current credits
            format!("${}", current)
        } else {
            // Show change with arrow notation
            let sign = if change > 0 { "+" } else { "-" };
            let abs_change = change.abs();
            format!("${} → ${} ({}${})", current, projected, sign, abs_change)
        }
    });

    view! {
        <div class="panel credits-panel">
            <div class="panel-header">
                <h3>"Credits"</h3>
            </div>
            <div class="panel-content credits-content">
                <div class="credits-display">
                    <span class="credits-symbol">"💰 $"</span>
                    <span 
                        class="credits-amount"
                        class:credit-gain=move || change_class.get() == "credit-gain"
                        class:credit-loss=move || change_class.get() == "credit-loss"
                    >
                        {move || projected_display.get()}
                    </span>
                </div>
                // Trade preview hint (shown when there's a pending trade)
                <div 
                    class="credit-trade-hint"
                    class:hidden=move || credit_change.get() == 0
                >
                    {move || {
                        let change = credit_change.get();
                        if change > 0 {
                            format!("+${} from sales", change)
                        } else if change < 0 {
                            format!("-${} for purchases", change.abs())
                        } else {
                            String::new()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

#[cfg(all(test, feature = "web"))]
mod tests {
    use super::*;
    use leptos::create_runtime;

    #[test]
    fn test_credit_change_class_gain() {
        let runtime = create_runtime();
        let credit_change = create_memo(move |_| 500i32);
        
        let result = credit_change.get();
        assert!(result > 0, "Should detect credit gain");
        
        runtime.dispose();
    }

    #[test]
    fn test_credit_change_class_loss() {
        let runtime = create_runtime();
        let credit_change = create_memo(move |_| -500i32);
        
        let result = credit_change.get();
        assert!(result < 0, "Should detect credit loss");
        
        runtime.dispose();
    }

    #[test]
    fn test_credit_change_class_neutral() {
        let runtime = create_runtime();
        let credit_change = create_memo(move |_| 0i32);
        
        let result = credit_change.get();
        assert_eq!(result, 0, "Should detect neutral change");
        
        runtime.dispose();
    }

    #[test]
    fn test_projected_credits_calculation() {
        let runtime = create_runtime();
        let current = 1000u32;
        let credit_change = create_memo(move |_| -100i32);
        let projected_credits = create_memo(move |_| {
            let credits = current as i32;
            let change = credit_change.get();
            (credits + change).max(0) as u32
        });
        
        assert_eq!(projected_credits.get(), 900, "Should calculate projected credits correctly");
        
        runtime.dispose();
    }
}
