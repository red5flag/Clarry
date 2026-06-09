use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::seed_metrics;

pub fn page_token() -> impl IntoToken {
    let metrics = seed_metrics();

    col()
        .id("dashboard_page")
        .css("min-h-screen bg-gray-50")
        // Header
        .child(row()
            .css("sticky top-0 z-50 bg-white border-b px-6 py-4 items-center justify-between")
            .child(text("Dashboard").css("text-2xl font-bold"))
            .child(row().css("gap-4")
                .child(text("🔔").css("text-xl"))
                .child(img_block("https://i.pravatar.cc/150?u=me")
                    .css("w-8 h-8 rounded-full")
                )
            )
        )
        // Metrics grid
        .child(grid(2).css("gap-4 px-6 py-6")
            .children(metrics.iter().map(|m| {
                col()
                    .css("bg-white rounded-xl p-5 shadow-sm gap-2")
                    .child(text(m.label.as_str()).css("text-sm text-gray-500"))
                    .child(text(m.value.as_str()).css("text-2xl font-bold"))
                    .child(row().css("items-center gap-1")
                        .child(text(if m.is_positive { "↑" } else { "↓" })
                            .css(if m.is_positive { "text-green-500 text-sm" } else { "text-red-500 text-sm" }))
                        .child(text(format!("{:.1}%", m.change.abs()))
                            .css(if m.is_positive { "text-green-600 text-sm" } else { "text-red-600 text-sm" }))
                    )
            }))
        )
        // Chart placeholder
        .child(block()
            .css("mx-6 bg-white rounded-xl p-5 shadow-sm")
            .child(text("Revenue Trend").css("text-lg font-bold mb-4"))
            .child(block()
                .css("h-40 bg-gradient-to-r from-blue-100 to-blue-50 rounded-lg flex items-end px-4 pb-4 gap-2")
                .children([40, 65, 45, 80, 55, 90, 70].iter().map(|&h| {
                    block().css("flex-1 bg-blue-500 rounded-t-sm")
                        .css(format!("height:{}%", h))
                }))
            )
        )
        // Bottom nav
        .child(row()
            .css("fixed bottom-0 left-0 right-0 bg-white border-t py-3 px-8 justify-between")
            .child(text("📊").css("text-xl text-blue-600"))
            .child(text("📋").css("text-xl"))
            .child(text("👥").css("text-xl"))
            .child(text("⚙").css("text-xl"))
        )
}
