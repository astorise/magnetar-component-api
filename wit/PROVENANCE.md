# Where each released version came from

The contract was written in Magnetar's `magnetar-runtime/wit/` before this
repository existed. Each directory here holds the content its version had at
the last commit that changed it in that history. Several versions only ever
lived in the branch of a pull request, so their content was cross-checked:
1.8.0, 1.9.0, 1.10.0, 1.12.0 and 1.14.0 are byte-identical to the copy a Model
Component carries (Gemma, Llama, Mistral and OLMo for 1.8.0; SmolLM; DeepSeek;
ModernBERT; Qwen), and 1.11.0 is byte-identical to the file its successor
1.12.0 was written from (the parent commit of 1.12.0's).

Versions 1.0.0 and 1.1.0 are not here: the Runtime serves 1.2.0 and later.

| Version | Date | Magnetar commit | Subject |
| --- | --- | --- | --- |
| 1.2.0 | 2026-09-11 | [`9d57ad4`](https://github.com/astorise/Magnetar/commit/9d57ad4a064dfbfedccbe7360a2216c36b3e19ee) | feat: real QKV attention bias support, complete and archive production Qwen loading |
| 1.3.0 | 2026-09-22 | [`89cbd2c`](https://github.com/astorise/Magnetar/commit/89cbd2c9b29d73e449847893346cf0e6a4277f1a) | Fix coverage ratchet regression from #75 (#91) |
| 1.4.0 | 2026-11-06 | [`3a74f07`](https://github.com/astorise/Magnetar/commit/3a74f0768fa768cbc1973b3332e9305a78c21c90) | Qualify Llama-3.2-1B-Instruct through the Llama Model Component (model-config 1.4.0 rope-scaling) (#164) |
| 1.5.0 | 2026-11-12 | [`12a58e9`](https://github.com/astorise/Magnetar/commit/12a58e9ef5f2cf67768e399bc0fb58fb85f80420) | Quantized GGUF weights on CUDA and CPU (Q2–Q8, IQ2–IQ4), multi-GPU, MoE (#167) |
| 1.6.0 | 2026-11-15 | [`2bb6eb6`](https://github.com/astorise/Magnetar/commit/2bb6eb6de91fdcc76923e5765c6be798a697ef39) | Expose a per-layer layout through model-config 1.6.0 (add-layer-layout-model-config) (#192) |
| 1.7.0 | 2026-11-16 | [`2cf2769`](https://github.com/astorise/Magnetar/commit/2cf276990fd924bb91bf0eef792bb317024c0f8d) | Express multi-head latent attention, mixed expert layers and YaRN (model-config 1.7.0) (#201) |
| 1.8.0 | 2026-11-21 | [`d736505`](https://github.com/astorise/Magnetar/commit/d7365050c0a1bc8a3f62e9dc6638cf510a63c998) | Carry decoder activation and add geodesic residual primitives |
| 1.9.0 | 2026-11-21 | [`457e0be`](https://github.com/astorise/Magnetar/commit/457e0be0963787b98903f990a3d81b19ed377e72) | Support per-layer rotary selection for SmolLM3 |
| 1.10.0 | 2026-11-21 | [`2173fbd`](https://github.com/astorise/Magnetar/commit/2173fbd16b6fbe28b2219c696d967b0c2d58fc07) | Run grouped GLM expert routing end to end |
| 1.11.0 | 2026-11-21 | [`f7ee4e8`](https://github.com/astorise/Magnetar/commit/f7ee4e8d94cda3b606c14df4c331f117c7b27880) | Allow component graphs to declare multiple outputs |
| 1.12.0 | 2026-11-21 | [`4a18990`](https://github.com/astorise/Magnetar/commit/4a18990efc725494d234bc7adb223ebc23e74a84) | Add a component-produced bidirectional encoder graph |
| 1.13.0 | 2026-11-25 | [`ee59d7c`](https://github.com/astorise/Magnetar/commit/ee59d7ce35d53f4d9a5597fe37f5c09b4dc89fa6) | Run Qwen3.5's hybrid decoder on CPU and CUDA; Qwen4-Exp operators; CPU decode work; accelerator-package RFC (#231) |
| 1.14.0 | 2026-11-25 | [`a6bcedd`](https://github.com/astorise/Magnetar/commit/a6bcedda134b65ce1580082093f9bace06578eb8) | Run Qwen4-Exp's blocks on the CPU Providers and against Transformers (model-config 1.14.0) (#234) |
| 1.15.0 | 2026-11-27 | [`daec9e2`](https://github.com/astorise/Magnetar/commit/daec9e204401f96f05db5e8e96fe1481a528dd0a) | Ordered Mamba-2 hybrid decoder configuration, proposed in Magnetar #252. |
| 1.16.0 | 2026-11-27 | [Magnetar #252](https://github.com/astorise/Magnetar/pull/252) | Runtime-bound copy-based KV history inputs and complete-history outputs. |

From this repository on, a version is added here first; `SHA256SUMS` records
its checksum and the files of a recorded version are never edited.
