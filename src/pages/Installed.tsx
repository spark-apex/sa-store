/// 已安装页 — 列出本地已安装的 SA 产品
/// 后续接入 Tauri 本地检测，目前展示占位 UI

export default function Installed() {
    /// 本地已安装的应用列表（后续从本地文件系统检测）
    const localApps = [
        { name: 'SA-AutoClick', version: '2.0.0', key: 'sa-auto-click' },
        { name: 'SA-Store', version: '1.0.0', key: 'sa-store' },
    ]

    return (
        <div class="p-4 animate-in">
            <div class="flex-between mb-4">
                <span class="font-semibold text-sm" style={{ color: 'var(--text-primary)' }}>已安装应用</span>
                <button class="text-xs px-3 py-1.5 rounded-lg border-none cursor-pointer"
                    style={{ background: 'var(--brand)', color: 'white' }}>
                    全部更新
                </button>
            </div>

            <div class="flex flex-col gap-2.5">
                {localApps.map((app) => (
                    <div class="flex items-center gap-3 p-3 rounded-xl"
                        style={{
                            background: 'var(--bg-card)',
                            border: '1px solid var(--border-subtle)',
                        }}>
                        {/* Logo */}
                        <div class="w-10 h-10 rounded-lg flex-center flex-shrink-0"
                            style={{ background: 'linear-gradient(135deg, var(--brand), var(--brand-light, #80D8FF))' }}>
                            <span class="text-white font-bold text-sm">{app.name.charAt(3)}</span>
                        </div>

                        {/* 信息 */}
                        <div class="flex-1 min-w-0">
                            <div class="font-medium text-sm" style={{ color: 'var(--text-primary)' }}>{app.name}</div>
                            <div class="text-xs mt-0.5" style={{ color: 'var(--text-ghost)' }}>v{app.version}</div>
                        </div>

                        {/* 状态 */}
                        <div class="flex items-center gap-1 text-xs" style={{ color: '#10b981' }}>
                            <div class="i-carbon-checkmark-filled text-3" />
                            已最新
                        </div>
                    </div>
                ))}
            </div>
        </div>
    )
}
