/// 推荐页 — 应用商店首页
import { createSignal, createResource, For, Show } from 'solid-js'
import { fetchApps } from '@/services/api'
import type { AppSummary } from '@/services/api'
import AppCard from '@/components/AppCard'

export default function Home() {
    const [apps] = createResource(fetchApps)

    return (
        <div class="p-4 animate-in">
            {/* 欢迎横幅 */}
            <div class="rounded-2xl p-5 mb-4"
                style={{
                    background: 'linear-gradient(135deg, #1296DB, #80D8FF)',
                    'box-shadow': '0 4px 20px rgba(18, 150, 219, 0.3)',
                }}>
                <div class="text-white font-bold text-lg">星火极点应用商店</div>
                <div class="text-white/70 text-xs mt-1">发现优质应用，一键安装更新</div>
            </div>

            {/* 官方应用 */}
            <div class="mb-4">
                <div class="flex-between mb-3">
                    <span class="font-semibold text-sm" style={{ color: 'var(--text-primary)' }}>官方应用</span>
                    <span class="text-xs cursor-pointer" style={{ color: 'var(--brand)' }}>查看全部</span>
                </div>

                <Show when={apps.loading}>
                    <div class="flex-center py-8">
                        <div class="w-5 h-5 border-2 rounded-full animate-spin"
                            style={{ 'border-color': 'var(--border)', 'border-top-color': 'var(--brand)' }} />
                    </div>
                </Show>

                <Show when={apps()}>
                    <div class="flex flex-col gap-2.5">
                        <For each={apps()!.filter(a => a.is_official)}>
                            {(app) => <AppCard app={app} />}
                        </For>
                    </div>
                </Show>

                <Show when={!apps.loading && apps()?.filter(a => a.is_official).length === 0}>
                    <div class="text-center py-8 text-sm" style={{ color: 'var(--text-ghost)' }}>
                        暂无已发布的官方应用
                    </div>
                </Show>
            </div>

            {/* 第三方应用 */}
            <Show when={apps()?.filter(a => !a.is_official).length}>
                <div class="mb-4">
                    <div class="flex-between mb-3">
                        <span class="font-semibold text-sm" style={{ color: 'var(--text-primary)' }}>第三方应用</span>
                    </div>
                    <div class="flex flex-col gap-2.5">
                        <For each={apps()!.filter(a => !a.is_official)}>
                            {(app) => <AppCard app={app} />}
                        </For>
                    </div>
                </div>
            </Show>
        </div>
    )
}
