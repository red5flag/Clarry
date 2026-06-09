
🌐 Clarry; Edge-First Web Cloning & Generation Engine

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)] [![Leptos](https://img.shields.io/badge/Leptos-0.8-blue.svg)] [![Tract-ONNX](https://img.shields.io/badge/Inference-Tract--ONNX-green.svg)] [![SmolVLM](https://img.shields.io/badge/Model-SmolVLM--256M-purple.svg)] [![Edge-Optimized](https://img.shields.io/badge/Edge-Medium--End-lightgrey.svg)] [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)]

📖 Overview
Clarry is an open-source, model-powered webcrawler, cloner, builder, and exporter designed for frontend R&D and edge AI research. It transforms any webpage—live, archived, or from a single screenshot—into production-ready Leptos/Rust + Tailwind CSS + HTML using a structured, self-rendering token system.

Built for medium-end edge hardware, Clarry runs fully local inference via tract-onnx and SmolVLM-50M via deterministic, pre-tokenized component trees that compile to fast, memory-safe WASM/SSR applications.

🔄 Core Pipeline
📸 Screenshot / URL 
   ↓
🔍 VLM Detection + Tokenized Webcrawling
   ↓
🧩 Token Encoder → Token Tree
   ↓
🧠 SmolVLM-256M + Tract-ONNX 
   ↓
⚙️ Codegen → Leptos `.rs` + Tailwind + Inline CSS
   ↓
📦 Export / Dataset → JSONL, PNG, WASM, SSR Modules


Features
🕷️ Smart Webcrawler
Selects semantic key elements, strips noise, extracts asset URLs, DOM hints, and layout hints

🧠 Edge-Optimized Inference
Runs SmolVLM-50 locally via tract-onnx on CPU/NPU. <512MB RAM footprint

🧩 300+ Pre-Tokenized Primitives	
Flex, Grid, Cards, Lists, InfiniteScroll, Suspense, Forms, Modals, WebGPU, PWA, A11y, Transitions, etc.

🛠️ Token-Based Builder	
Drag-and-drop visual editor with real-time Rust/Tailwind output. Every token self-decodes & self-renders

📦 Universal Cloner	
Reconstructs legacy & modern sites (Instagram, MySpace, SaaS dashboards, e-commerce, blogs)

📊 Synthetic Dataset Engine
Generates 10k+ randomized PNG + JSONL variants for VLM fine-tuning & self-improving pipelines

🧩 Token Architecture
Clarry replaces monolithic HTML parsers with a deterministic token graph. Every UI primitive is a self-contained LayoutNode variant that handles its own decoding, styling, and Leptos rendering.

pub enum LayoutKind {
    FlexRow, FlexCol, Grid, Card, Modal, InfiniteScroll,
    PostCard, StoryViewer, ReelPlayer, ChatThread, SearchResults,
    // ... 300+ variants
}

pub struct LayoutNode {
    pub layout: LayoutKind,
    pub style: StyleToken,
    pub events: Option<Vec<EventBindingToken>>,
    pub behavior: Option<HtmlBehaviorToken>,
    pub resource_hints: Option<Vec<ResourceHintToken>>,
    pub transitions: Option<TransitionToken>,
    pub children: Vec<LayoutNode>,
}

Why Tokens?
    ✅ SmolVLM-Ready: Model outputs standard JSON → serde decodes → render() emits Leptos
    ✅ Self-Contained: Each file = struct + impl Decode + impl Render. No monolithic dispatcher.
    ✅ Composable: Tokens nest infinitely. Complex pages are just tree merges.
    ✅ Deterministic: Same JSON → same DOM every time. Perfect for testing & fine-tuning.

🖥️ Edge Deployment & Hardware Specs
Clarry is engineered for medium-end edge devices (no GPU required).

Specs
Target
CPU: ARM64 (Raspberry Pi 4/5), x86_64 (Intel NUC, M-series, Jetson Nano)

RAM: 4GB recommended, 2GB minimum (quantized pipeline)

Inference: tract-onnx (CPU-optimized, SIMD Wide Optimization, optional INT8 quantization)

Model: SmolVLM-50M (ONNX)

Runtime: Axum SSR + Leptos WASM hydration, zero cloud calls

