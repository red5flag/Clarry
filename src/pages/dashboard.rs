use crate::tokens::builders::prelude::*;
use crate::tokens::node::IntoToken;
use crate::data::app_data::seed_metrics;

let metrics = seed_metrics();

col
        id dashboard_page
        css min-h-screen bg-gray-50
        row
            css sticky top-0 z-50 bg-white border-b px-6 py-4 items-center justify-between
            txt "Dashboard"
                css text-2xl font-bold
            row
                css gap-4
                txt "User"
                    css text-xl
                img_block "https://i.pravatar.cc/150?u=me"
                    css w-8 h-8 rounded-full
        grid2
            css gap-4 px-6 py-6
            .add_all(metrics.iter().map(|m| {
                col()
                    .set_class("bg-white rounded-xl p-5 shadow-sm gap-2")
                    .add(text(m.label.as_str()).set_class("text-sm text-gray-500"))
                    .add(text(m.value.as_str()).set_class("text-2xl font-bold"))
                    .add(row().set_class("items-center gap-1")
                        .add(text(if m.is_positive { "↑" } else { "↓" })
                            .set_class(if m.is_positive { "text-green-500 text-sm" } else { "text-red-500 text-sm" }))
                        .add(text(format!("{:.1}%", m.change.abs()))
                            .set_class(if m.is_positive { "text-green-600 text-sm" } else { "text-red-600 text-sm" }))
                    )
            }))
        block
            css mx-6 bg-white rounded-xl p-5 shadow-sm
            txt "Revenue Trend"
                css text-lg font-bold mb-4
            block
                css h-40 bg-gradient-to-r from-blue-100 to-blue-50 rounded-lg flex items-end px-4 pb-4 gap-2
                .add_all([40, 65, 45, 80, 55, 90, 70].iter().map(|&h| {
                    block().set_class("flex-1 bg-blue-500 rounded-t-sm")
                        .set_class(format!("height:{}%", h))
                }))
        row
            css fixed bottom-0 left-0 right-0 bg-white border-t py-3 px-8 justify-between
            txt "📊"
                css text-xl text-blue-600
            txt "📋"
                css text-xl
            txt "👥"
                css text-xl
            txt "⚙"
                css text-xl
