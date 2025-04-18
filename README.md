# Rust Tutorial

## Fundamental

- `rustup` : 安裝 `rustc`, `cargo`, `rustup` 與其他工具在 `$HOME/.cargo/bin` 路徑下。
- 工具鏈檢查
  檢查工具鏈，確認工具鏈版本，有時需要配合 `Cargo.toml`

  ```bash
  cargo toolchain list
  ```
- 添加套件

  1. Rust 的套件網站是 [`crates.io`](https://crates.io/)
  2. 搜尋套件並添加到專案的 `Cargo.toml`
     ```toml
     [dependencies]
     ```
  3. 執行 `cargo build`，第一次會看到套件加載; 第二次執行就不會看到加載套件，因為已經存在 `Cargo.lock`
- 專案文件

  ```bash
  cargo doc --open
  ```

## Cargo

- `cargo build` 使用 `dev` 設定檔、`cargo build --release` 使用 `release` 設定檔
- 從 `Cargo.toml` 中設定 `[profile.*]`
  ```toml
  [profile.dev]
  opt-level = 0
  [profile.release]
  opt-level = 3
  ```
- 用 `cargo publish` 可以發布 crate，關於 crate 的[文件註解方式](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)

## Ownership

藉由 ownership 特性，Rust 不需要垃圾收集器就能保證記憶體安全。

> 具有垃圾收集器 (GC) 的語言，在程式運行時，定期查找不再使用的記憶體並釋放。
> 沒有垃圾收集器的語言，需要明確分配、釋放記憶體。

### Stack & Heap

**Stack**

- Last In First Out (LIFO)
- 儲存在 stack 上的資料必須要有已知的固定大小，若是未知或大小可能改變，則需要放在 heap 上

**Heap**

- 向記憶體要求一定的空間，記憶體分配器會找到足夠的空間並標記為使用，返回一個指針 (pointer)
- 發送資料到 stack 比 heap 還要快，因為分配器不需要找空間

### Ownership Rules

- 每個 value 都有自己的 owner
- 同一時間只能有一個 owner
- 當 owner 超出自己的範圍 (scope)，其 value 會被刪除

### Race Condition

Rust 可以在編譯時防止 race condition (競爭條件)。

- 兩個或多個 pointer 同時存取相同資料
- 至少有一個 pointer 正在寫入資料
- 沒有任何機制同步對資料的存取

### Features

- `Drop` 跟 `Copy` 特性不會同時存在
- 一個東西不能同時獲得 immutable reference 與 mutable reference
- Rust 自動防止 dangling reference (指向已分配給其他物件的記憶體位址的指標)
- Slice 更像一種 reference，沒有 ownership

## Project

一個 Rust 專案由以下四個元件組成 :

- Packages
- Crates
- Modules
- Paths

**Crates**

- crate 是 Rust 編譯器運行的最小單位量
- crate 有兩種形式 : binary or library
  - binary 有 `main`
  - library 為多個項目共享的功能
- 創建 library crate 使用 `cargo new <package_name> --lib`

**Packages**

- 至少一個 crates 組成，且包含一個 `Cargo.toml`

### Syntax

- 用 `pub` 公開
- `mod` 代表模組
- enum 預設是公開的，即 `pub`
- struct 預設是不公開的

## Lifetime

> - `&i32` is a reference
> - `&'a i32` is a reference with an explicit lifetime
> - `&'a mut i32` is a mutable reference with an explicit lifetime

- Functions 或 structs 使用 references 不需要特別標明 lifetime annotation `'a`，Rust 預設可以通過編譯
- 編譯器為 function 的每個 reference 參數指派一個 lifetime 參數 `'a`
- 如果 function 輸入只有一個 lifetime 參數，則該 lifetime 將指派給所有輸出 lifetime 參數
- 如果 function 的多個輸入中包含 `&self` 或 `&mut self`，則 `self` 被指派給所有輸出 lifetime 參數
- `&'static` 靜態生命週期的 reference，資料會存活整個程式期間

## Smart Pointer

Smart Pointer 除了像 reference poitner 一樣運作外，還具有 metadata 和其他功能，並且擁有 pointer 指向的資料。

### `Rc<T>` (Reference Counting)

引用技術智慧指標 (reference-counted smart pointer)，用於在**單執行緒**中實現多個所有者共享同一數據。透過追蹤 `strong_count` 管理數據的生命週期。

> 多執行續 (並發) 狀況下，需要使用 `Arc<T>`

- 創建一個 `Rc<T>` (by `Rc::new()`)，`strong_count` 為 1
- 每次呼叫 `Rc::clone(&rc)` 時，`strong_count` 加 1
- 當一個 `Rc<T>` 超出作用域時，`strong_count` 減 1
- 當 `strong_count` 為 0 時，內部數據被釋放 (`Drop` trait)

> `Rc<T>` 提供不可變訪問，無法直接修改內部的 `T`，除非 `T` 支援如 `RefCell<T>`

### `RefCell<T>`

- `RefCell<T>` 使用在當我們需要改變不可變類型的內部值時
- `RefCell<T>` 維護借用狀態 (borrow state)，紀錄目前有多少不可變借用數量 (透過 `borrow()` 獲得的 `Ref<T>`)
- `RefCell<T>` 可以擁有多個不可變借用或一個可變借用 (透過 `borrow_mut()` 獲得的 `RefMut<T>`)

### `Weak<T>`

`Rc<T>` 實例會計算 `strong_count`，當 `strong_count` 為 0 時，將 `Rc<T>` 進行回收。但 `strong_count` 可能不會為 0，因此實例不會被釋放，即循環引用 (reference cycles)。

將 `Rc<T>` 改用 `Weak<T>` 來防止循環引用，呼叫 `Rc::downgrade` 來獲得 `Weak<T>` smart pointer。`Weak<T>` 會增加 `weak_count`，`weak_count` 不需要為 0，只要 `strong_count`為 0 `Rc<T>`也能被清理。但 `Weak<T>` 所引用的值可能不存在，所以要對該值進行操作前，需要先 `upgrade` 確認值是否存在，回傳 `Option<Rc<T>>`。

## Threads

- `thread::spawn()` 創建一個新執行緒
- `<thread>.join().unwrap()` 來等待 thread 執行結束
- 用 `move` 使得閉包取得正確的 ownership
- `Send` trait 可以讓執行緒間轉移所有權 (不需要手動實現)
- `Sync` trait 可用於與多個現成共享訪問 (不需要手動實現)

### `Arc<T>`

- 原子引用計數類型 (atomically reference counted type)
- thread-safe 會帶來效能損失，所以 standard library 沒有預設使用

## Trivial Concept

- Rust 中不存在 `null` 值，但可以透過 `Option<T>` 表示

  ```rust
  enum Option<T> {
    None,
    Some(T),
  }
  ```
- `println!("{}")` vs `println!("{:?}")`

  | 特性    | `println!("{}")`       | `println!("{:?}")`      |
  | ------- | ------------------------ | ------------------------- |
  | Library | `std::fmt::Display`    | `std::fmt:Debug`        |
  | 實現    | 需要手動實現 `Display` | 加上 `#[derive(Debug)]` |
- 使用 `RUST_BACKTRACE=1` 來顯示 `cargo run` 的錯誤調試訊息
- panic 處理方法可以使用 `unwrap()` 或是 `expect()`
- `Box<dyn Error>` 回傳任何錯誤的方法

  - 不能**靜態確定**，只能在運作時知道
  - `Box<T>` 是 Heap 分配的 pointer
  - `dyn Error` 代表實作任何 `Error` trait 的錯誤型別
- Smart Pointer 除了像 reference poitner 一樣運作外，還具有 metadata 和其他功能，並且擁有 pointer 指向的資料
- `RefCell<T>` 追蹤目前有多少 `Ref<T>` 和 `RefMut<T>`，每次呼叫 `borrow()` 時，`RefCell<T>` 增加活躍的不可變借貸數量
- `RefCell<T>` 可以擁有多個不可變借用或一個可變借用
- 可以將 `Rc<T>` 改用 `Weak<T>` 來防止循環引用

  - `Rc<T>` 實例會計算 `strong_count`，當 `strong_count` 為 0 時，將 `Rc<T>` 進行回收
  - 循環引用的狀況是，`strong_count` 不會為 0，因此實例不會被釋放
  - 呼叫 `Rc::downgrade` 來獲得 `Weak<T>` smart pointer
  - `Weak<T>` 會增加 `weak_count`，不需要為 0 `Rc<T>`也能被清理
  - 但 `Weak<T>` 所引用的值可能不存在，所以要對該值進行操作前，需要先 `upgrade` 確認值是否存在，回傳 `Option<Rc<T>>`
  -
