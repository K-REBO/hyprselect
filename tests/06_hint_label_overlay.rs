// ヒントラベルオーバーレイ統合テスト
// このテストは、hyprelectのコア機能を手動で検証します：
//   1. Hyprland IPCで可視タイルを取得
//   2. 各タイルにヒント文字を割り当て
//   3. 画面全体オーバーレイにCairoでヒントラベルを描画
//   4. キーボード入力を受け付け、対応するタイルをフォーカス
//
// 実行方法:
//   cargo test --test 06_hint_label_overlay --features hyprland,wayland -- --nocapture
//
// 前提条件:
//   - Waylandコンポジタ（Hyprland）が起動していること
//   - 複数のタイルウィンドウが表示されていること
//
// 操作方法:
//   - 表示されたヒント文字を押すと、対応するウィンドウにフォーカスが移ります
//   - Escapeキーでキャンセルします

use anyhow::{Context, Result};
use std::os::fd::AsFd;

use hyprland::data::{Clients, Monitors};
use hyprland::dispatch::{Dispatch as HyprDispatch, DispatchType, WindowIdentifier};
use hyprland::prelude::*;

use wayland_client::{
    Connection, Dispatch, QueueHandle,
    globals::{GlobalListContents, registry_queue_init},
    protocol::{
        wl_buffer, wl_compositor, wl_keyboard, wl_registry, wl_seat, wl_shm, wl_shm_pool,
        wl_surface,
    },
};

use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, Layer},
    zwlr_layer_surface_v1::{self, Anchor, KeyboardInteractivity},
};

use xkbcommon::xkb;

const HINT_CHARS: &[char] = &['s', 'a', 'd', 'f', 'j', 'k', 'l', 'w', 'e', 'r', 'u', 'i'];

#[derive(Clone)]
struct TileInfo {
    address: hyprland::shared::Address,
    title: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    hint: char,
}

struct AppState {
    configured: bool,
    xkb_context: xkb::Context,
    xkb_state: Option<xkb::State>,
    selected_address: Option<hyprland::shared::Address>,
    should_exit: bool,
    tiles: Vec<TileInfo>,
}

impl AppState {
    fn new(xkb_context: xkb::Context, tiles: Vec<TileInfo>) -> Self {
        Self {
            configured: false,
            xkb_context,
            xkb_state: None,
            selected_address: None,
            should_exit: false,
            tiles,
        }
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for AppState {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        _: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &wl_compositor::WlCompositor,
        _: wl_compositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &wl_surface::WlSurface,
        _: wl_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_shm::WlShm, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &wl_shm::WlShm,
        _: wl_shm::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_shm_pool::WlShmPool, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &wl_shm_pool::WlShmPool,
        _: wl_shm_pool::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_buffer::WlBuffer, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &wl_buffer::WlBuffer,
        _: wl_buffer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &wl_seat::WlSeat,
        _: wl_seat::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<zwlr_layer_shell_v1::ZwlrLayerShellV1, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &zwlr_layer_shell_v1::ZwlrLayerShellV1,
        _: zwlr_layer_shell_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1, ()> for AppState {
    fn event(
        state: &mut Self,
        layer_surface: &zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
        event: zwlr_layer_surface_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let zwlr_layer_surface_v1::Event::Configure { serial, .. } = event {
            layer_surface.ack_configure(serial);
            state.configured = true;
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for AppState {
    fn event(
        state: &mut Self,
        _: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            wl_keyboard::Event::Keymap { format, fd, size } => {
                if format == wayland_client::WEnum::Value(wl_keyboard::KeymapFormat::XkbV1) {
                    let keymap = unsafe {
                        let ptr = nix::sys::mman::mmap(
                            None,
                            std::num::NonZeroUsize::new(size as usize).unwrap(),
                            nix::sys::mman::ProtFlags::PROT_READ,
                            nix::sys::mman::MapFlags::MAP_PRIVATE,
                            fd.as_fd(),
                            0,
                        )
                        .expect("mmapに失敗");

                        let slice =
                            std::slice::from_raw_parts(ptr.as_ptr() as *const u8, size as usize - 1);
                        let keymap_str = std::str::from_utf8_unchecked(slice);
                        let keymap = xkb::Keymap::new_from_string(
                            &state.xkb_context,
                            keymap_str.to_string(),
                            xkb::KEYMAP_FORMAT_TEXT_V1,
                            xkb::KEYMAP_COMPILE_NO_FLAGS,
                        )
                        .expect("キーマップの作成に失敗");

                        nix::sys::mman::munmap(ptr, size as usize).expect("munmapに失敗");
                        keymap
                    };

                    state.xkb_state = Some(xkb::State::new(&keymap));
                }
            }

            wl_keyboard::Event::Key {
                key,
                state: key_state,
                ..
            } => {
                if let Some(xkb_state) = &mut state.xkb_state {
                    let keycode = key + 8;
                    if let wayland_client::WEnum::Value(wl_keyboard::KeyState::Pressed) = key_state
                    {
                        let keysym = xkb_state.key_get_one_sym(xkb::Keycode::from(keycode));

                        if keysym == xkb::keysyms::KEY_Escape.into() {
                            println!("\nEscapeキー: キャンセルします");
                            state.should_exit = true;
                            return;
                        }

                        let keysym_name = xkb::keysym_get_name(keysym);
                        let pressed_char = keysym_name.chars().next();

                        if let Some(ch) = pressed_char {
                            if let Some(tile) = state.tiles.iter().find(|t| t.hint == ch) {
                                println!("\nキー '{}' が押されました → '{}'", ch, tile.title);
                                state.selected_address = Some(tile.address.clone());
                            } else {
                                println!("\nキー '{}': ヒントに一致しません。キャンセルします。", ch);
                            }
                        }

                        state.should_exit = true;
                    }
                }
            }

            wl_keyboard::Event::Modifiers {
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
                ..
            } => {
                if let Some(xkb_state) = &mut state.xkb_state {
                    xkb_state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);
                }
            }

            _ => {}
        }
    }
}

fn draw_hint_overlay(
    shm: &wl_shm::WlShm,
    qh: &QueueHandle<AppState>,
    screen_width: i32,
    screen_height: i32,
    tiles: &[TileInfo],
) -> Result<wl_buffer::WlBuffer> {
    let stride = screen_width * 4;
    let size = stride * screen_height;

    let cairo_surface =
        cairo::ImageSurface::create(cairo::Format::ARgb32, screen_width, screen_height)
            .context("Cairo ImageSurfaceの作成に失敗")?;

    {
        let cr = cairo::Context::new(&cairo_surface).context("Cairo Contextの作成に失敗")?;

        // 全体を透明に
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        cr.paint().context("背景クリアに失敗")?;

        for tile in tiles {
            let label = tile.hint.to_string();

            // ヒントボックスの背景（タイル左上）
            let box_w = 80.0_f64;
            let box_h = 80.0_f64;
            let bx = tile.x as f64;
            let by = tile.y as f64;

            cr.set_source_rgba(0.12, 0.12, 0.12, 0.92);
            cr.rectangle(bx, by, box_w, box_h);
            cr.fill().context("ボックス背景描画に失敗")?;

            // ヒント文字
            cr.select_font_face("Mono", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
            cr.set_font_size(52.0);

            let extents = cr.text_extents(&label).context("テキストサイズ測定に失敗")?;
            let tx = bx + (box_w - extents.width()) / 2.0 - extents.x_bearing();
            let ty = by + (box_h - extents.height()) / 2.0 - extents.y_bearing();

            cr.set_source_rgb(0.87, 0.87, 0.87);
            cr.move_to(tx, ty);
            cr.show_text(&label).context("テキスト描画に失敗")?;
        }
    }

    let mut cairo_surface = cairo_surface;
    cairo_surface.flush();
    let cairo_data = cairo_surface.data().context("Cairoデータの取得に失敗")?;

    let temp_file = tempfile::tempfile().context("一時ファイルの作成に失敗")?;
    temp_file
        .set_len(size as u64)
        .context("ファイルサイズの設定に失敗")?;

    let mut mmap = unsafe {
        memmap2::MmapMut::map_mut(&temp_file).context("メモリマップに失敗")?
    };

    mmap.copy_from_slice(&cairo_data);
    drop(mmap);

    let pool = shm.create_pool(temp_file.as_fd(), size, qh, ());
    let buffer = pool.create_buffer(
        0,
        screen_width,
        screen_height,
        stride,
        wl_shm::Format::Argb8888,
        qh,
        (),
    );

    pool.destroy();
    Ok(buffer)
}

fn main() -> Result<()> {
    println!("=== hyprelectヒントラベルオーバーレイ統合テスト ===\n");

    // Step 1: Hyprland IPCで可視タイルを取得
    println!("【Step 1】Hyprlandから可視タイル情報を取得");
    println!("{}", "-".repeat(50));

    let clients = Clients::get().context("ウィンドウリストの取得に失敗")?;
    let client_vec = clients.to_vec();

    let monitors = Monitors::get().context("モニター情報の取得に失敗")?;
    let monitor_vec = monitors.to_vec();

    let visible_workspace_ids: Vec<i32> = monitor_vec
        .iter()
        .map(|m| m.active_workspace.id)
        .collect();

    let visible_clients: Vec<_> = client_vec
        .iter()
        .filter(|c| visible_workspace_ids.contains(&c.workspace.id))
        .filter(|c| !c.floating)
        .collect();

    if visible_clients.is_empty() {
        println!("⚠ 可視タイルがありません。テストを終了します。");
        return Ok(());
    }

    let tiles: Vec<TileInfo> = visible_clients
        .iter()
        .zip(HINT_CHARS.iter())
        .map(|(c, &hint)| {
            println!(
                "  [{}] {} - 位置({}, {}) サイズ{}x{}",
                hint, c.title, c.at.0, c.at.1, c.size.0, c.size.1
            );
            TileInfo {
                address: c.address.clone(),
                title: c.title.clone(),
                x: c.at.0 as i32,
                y: c.at.1 as i32,
                width: c.size.0 as i32,
                height: c.size.1 as i32,
                hint,
            }
        })
        .collect();

    let screen_width = monitor_vec
        .iter()
        .map(|m| m.width)
        .max()
        .unwrap_or(1920) as i32;
    let screen_height = monitor_vec
        .iter()
        .map(|m| m.height)
        .max()
        .unwrap_or(1080) as i32;

    println!("\n✓ {}個のタイルを取得（画面: {}x{}）", tiles.len(), screen_width, screen_height);

    // Step 2: Wayland接続とオーバーレイ作成
    println!("\n【Step 2】ヒントラベルオーバーレイを表示");
    println!("{}", "-".repeat(50));

    let conn = Connection::connect_to_env().context("Waylandへの接続に失敗")?;
    let (globals, mut event_queue) = registry_queue_init::<AppState>(&conn)
        .context("グローバルレジストリの取得に失敗")?;

    let qh = event_queue.handle();

    let compositor: wl_compositor::WlCompositor =
        globals.bind(&qh, 4..=6, ()).context("wl_compositorのバインドに失敗")?;
    let shm: wl_shm::WlShm =
        globals.bind(&qh, 1..=1, ()).context("wl_shmのバインドに失敗")?;
    let layer_shell: zwlr_layer_shell_v1::ZwlrLayerShellV1 =
        globals.bind(&qh, 1..=4, ()).context("zwlr_layer_shell_v1のバインドに失敗")?;
    let seat: wl_seat::WlSeat =
        globals.bind(&qh, 7..=9, ()).context("wl_seatのバインドに失敗")?;

    println!("✓ Waylandグローバルをバインド");

    let surface = compositor.create_surface(&qh, ());
    let layer_surface = layer_shell.get_layer_surface(
        &surface,
        None,
        Layer::Overlay,
        "hyprselect_hint_overlay".to_string(),
        &qh,
        (),
    );

    layer_surface.set_anchor(Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right);
    layer_surface.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
    layer_surface.set_exclusive_zone(-1);
    surface.commit();

    let xkb_context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    let mut state = AppState::new(xkb_context, tiles.clone());

    while !state.configured {
        event_queue.blocking_dispatch(&mut state)?;
    }

    println!("✓ Layer Surface設定完了");

    // ヒントラベルバッファを描画
    let buffer = draw_hint_overlay(&shm, &qh, screen_width, screen_height, &tiles)
        .context("ヒントラベル描画に失敗")?;

    surface.attach(Some(&buffer), 0, 0);
    surface.damage_buffer(0, 0, screen_width, screen_height);
    surface.commit();

    let _keyboard = seat.get_keyboard(&qh, ());
    event_queue.roundtrip(&mut state)?;

    println!("✓ ヒントラベルを表示");
    println!("\n【操作】ヒント文字を押してウィンドウを選択、Escapeでキャンセル");

    // Step 3: キーボード入力を待機
    while !state.should_exit {
        event_queue.blocking_dispatch(&mut state)?;
    }

    // クリーンアップ
    layer_surface.destroy();
    surface.destroy();
    event_queue.roundtrip(&mut state)?;

    // Step 4: フォーカス変更
    println!("\n【Step 3】フォーカス変更");
    println!("{}", "-".repeat(50));

    if let Some(addr) = state.selected_address {
        HyprDispatch::call(DispatchType::FocusWindow(WindowIdentifier::Address(addr)))
            .context("フォーカスの変更に失敗")?;
        println!("✓ フォーカス変更成功");
    } else {
        println!("キャンセル（フォーカス変更なし）");
    }

    println!("\n=== テスト完了 ===");
    Ok(())
}
