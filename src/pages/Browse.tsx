/// 浏览页 — 搜索和分类浏览
import { createSignal, createResource, For, Show } from 'solid-js'
import { fetchApps } from '@/services/api'
import AppCard from '@/components/AppCard'

export default function Browse() {
    const [search, setSearch] = createSignal('')
    const [apps] = createResource(fetchApps)

    /// 过滤应用
    const filtered = () => {
        const list = apps() || []
        const q = search().toLowerCase()
        if (!q) return list
        return list.filter(a =>
            a.name.toLowerCase().includes(q) ||
            a.app_key.includes(q) ||
            (a.description || '').toLowerCase().includes(q)
        )
    }

    return (
        <div class="p-4 animate-in">
            {/* 搜索框 */}
            <div class="relative mb-4">
                <div class="i-carbon-search absolute left-3 top-1/2 -translate-y-1/2 text-3.5"
                    style={{ color: 'var(--text-ghost)' }} />
                <input
                    type="text"
                    placeholder="搜索应用..."
                    value={search()}
                    onInput={(e) => setSearch(e.currentTarget.value)}
                    class="w-full pl-9 pr-4 py-2.5 rounded-xl border-none outline-none text-sm"
                    style={{
                        background: 'var(--bg-card)',
                        color: 'var(--text-primary)',
                        border: '1px solid var(--border)',
                    }}
                />
            </div>

            {/* 结果 */}
            <Show when={apps.loading}>
                <div class="flex-center py-8">
                    <div class="w-5 h-5 border-2 rounded-full animate-spin"
                        style={{ 'border-color': 'var(--border)', 'border-top-color': 'var(--brand)' }} />
                </div>
            </Show>

            <Show when={!apps.loading}>
                <div class="text-xs mb-3" style={{ color: 'var(--text-ghost)' }}>
                    共 {filtered().length} 个应用
                </div>
                <div class="flex flex-col gap-2.5">
                    <For each={filtered()}>
                        {(app) => <AppCard app={app} />}
                    </For>
                </div>
                <Show when={filtered().length === 0}>
                    <div class="text-center py-12 text-sm" style={{ color: 'var(--text-ghost)' }}>
                        未找到匹配的应用
                    </div>
                </Show>
            </Show>
        </div>
    )
}
