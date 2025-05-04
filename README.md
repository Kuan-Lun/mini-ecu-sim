# mini-ecu-sim

模擬車用 ECU 控制狀態機的 Rust 專案，結合 watchdog timer 與錯誤紀錄模組，模擬嵌入式控制邏輯與車載應用常見故障情境。

## 功能簡介

- 模擬車輛 ECU 狀態（Off / Idle / Running / Overheated / Error）
- 結合 Watchdog Timer，模擬超時重啟機制
- 支援自動錯誤紀錄（WatchdogTimeout、Overheated）
- 每筆錯誤紀錄附有 UTC 時間戳（RFC3339 格式）
- 錯誤紀錄可輸出為 JSON，支援保存至檔案

## 使用方法

### 安裝與執行

```bash
git clone https://github.com/Kuan-Lun/mini-ecu-sim.git
cd mini-ecu-sim
cargo run
```

### 執行輸出範例

```text
ECU State (1st update): Idle
ECU State (2nd update): Error
Error log in JSON:
{
  "history": [
    {
      "code": "WatchdogTimeout",
      "timestamp": "2025-06-19T07:34:57.884063400+00:00"
    }
  ]
}
```

## 📦 技術重點

| 模組             | 說明                          |
| -------------- | --------------------------- |
| `state.rs`     | ECU 狀態機與更新邏輯                |
| `sensor.rs`    | 模擬感測器資料（溫度、轉速）              |
| `watchdog.rs`  | 模擬 watchdog timeout，支援重設與檢查 |
| `error_log.rs` | 錯誤類型定義、時間戳紀錄、JSON 序列化輸出     |

## 📁 專案結構

```
mini-ecu-sim/
├── src/
|   ├── tests
|   |   ├── integration_test.rs
│   ├── main.rs
│   ├── state.rs
│   ├── sensor.rs
│   ├── watchdog.rs
│   └── error_log.rs
├── Cargo.toml
└── CHANGELOG.md
```

## 使用套件

- [`serde`](https://github.com/serde-rs/serde) – 資料序列化/反序列化
- [`serde_json`](https://github.com/serde-rs/json) – JSON 輸出
- [`chrono`](https://github.com/chronotope/chrono) – UTC 時間處理與格式化

你觀察得很仔細，沒錯，我在 `README.md` 的「使用套件」一節中確實**漏列了以下套件**：

- `embedded-hal`
- `embedded-hal-mock`
- `thiserror`

這些都是專案實際使用、且對功能邏輯有貢獻的套件，應該明確列出。以下是補全後的修正版本：

## 使用套件

| 套件名稱                                                                 | 功能說明                              |
| -------------------------------------------------------------------- | --------------------------------- |
| [`embedded-hal`](https://github.com/rust-embedded/embedded-hal)      | 抽象化硬體介面（sensor input、控制邏輯）        |
| [`embedded-hal-mock`](https://github.com/rust-embedded/embedded-hal) | 測試與模擬用 mock 環境，模擬感測器輸入            |
| [`thiserror`](https://github.com/dtolnay/thiserror)                  | 定義與管理錯誤型別（支援 `std::error::Error`） |
| [`serde`](https://github.com/serde-rs/serde)                         | 序列化 / 反序列化架構基礎                    |
| [`serde_json`](https://github.com/serde-rs/json)                     | JSON 輸出功能，格式化錯誤記錄                 |
| [`chrono`](https://github.com/chronotope/chrono)                     | 取得當前時間與格式化為 RFC3339 時戳            |

## 適合用途

- Rust 新手練習嵌入式邏輯與狀態機
- 電動車控制系統模擬與測試
- 嵌入式錯誤紀錄系統原型
- 求職時展示實作與架構設計能力

## 版本

目前版本：`v0.2.1`
詳見 [CHANGELOG.md](./CHANGELOG.md)

## 授權

本專案採用 [GNU General Public License v3](./LICENSE) 授權條款。
