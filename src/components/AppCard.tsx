/// 应用卡片组件 — 列表中的每个应用
import type { AppSummary } from '@/services/api'
import { formatDownloads } from '@/services/api'
import { Show } from 'solid-js'

export default function AppCard(props: { app: AppSummary }) {
    const a = () => props.app

    return (
        <div class="flex items-center gap-3 p-3 rounded-xl cursor-pointer transition-all duration-200"
            style={{
                background: 'var(--bg-card)',
                border: '1px solid var(--border-subtle)',
                'box-shadow': 'var(--shadow-card)',
            }}
            onMouseEnter={(e) => { e.currentTarget.style.transform = 'translateY(-1px)'; e.currentTarget.style.borderColor = 'var(--brand)' }}
            onMouseLeave={(e) => { e.currentTarget.style.transform = 'none'; e.currentTarget.style.borderColor = 'var(--border-subtle)' }}>

            {/* Logo 占位 */}
            <div class="w-12 h-12 rounded-xl flex-center flex-shrink-0"
                style={{ background: 'linear-gradient(135deg, var(--brand), var(--brand-light, #80D8FF))', 'box-shadow': '0 2px 8px rgba(18, 150, 219, 0.25)' }}>
                <span class="text-white font-bold text-lg">{a().name.charAt(0)}</span>
            </div>

            {/* 信息 */}
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-1.5">
                    <span class="font-semibold text-sm truncate" style={{ color: 'var(--text-primary)' }}>{a().name}</span>
                    <Show when={a().is_official}>
                        <span class="text-xs px-1.5 py-0.5 rounded-full font-medium"
                            style={{ background: 'rgba(18, 150, 219, 0.1)', color: 'var(--brand)', 'font-size': '9px' }}>官方</span>
                    </Show>
                </div>
                <div class="text-xs mt-0.5 truncate" style={{ color: 'var(--text-ghost)' }}>
                    {a().description || a().category}
                </div>
                <div class="flex items-center gap-3 mt-1 text-xs" style={{ color: 'var(--text-ghost)' }}>
                    <Show when={a().latest_version}>
                        <span>v{a().latest_version}</span>
                    </Show>
                    <span>{formatDownloads(a().total_downloads)} 次下载</span>
                </div>
            </div>

            {/* 操作按钮 */}
            <button class="px-3 py-1.5 rounded-lg text-xs font-medium border-none cursor-pointer transition-all"
                style={{ background: 'var(--brand)', color: 'white' }}>
                获取
            </button>
        </div>
    )
}
