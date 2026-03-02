/// SA-Store 根组件
/// TitleBar + 页面切换 + 底部导航
import { onMount, Switch, Match } from 'solid-js'
import { TitleBar, windowCtl } from '@sa/ui/desktop'
import NavBar from '@/components/NavBar'
import Home from '@/pages/Home'
import Browse from '@/pages/Browse'
import Installed from '@/pages/Installed'
import { currentPage } from '@/services/state'

/// 星火极点品牌 Logo（应用商店用主品牌色）
function StoreLogo() {
    return (
        <svg width="16" height="16" viewBox="0 0 64 64" fill="none">
            <defs>
                <linearGradient id="sl1" x1="0" y1="0" x2="1" y2="0">
                    <stop offset="0%" stop-color="#B8D4F0" />
                    <stop offset="100%" stop-color="#D0E4F8" />
                </linearGradient>
                <linearGradient id="sl2" x1="0" y1="0" x2="1" y2="0">
                    <stop offset="0%" stop-color="#8BBDE8" />
                    <stop offset="100%" stop-color="#B0D2F0" />
                </linearGradient>
                <linearGradient id="sl3" x1="0" y1="0" x2="1" y2="0">
                    <stop offset="0%" stop-color="#4A9DE0" />
                    <stop offset="100%" stop-color="#6FB5E8" />
                </linearGradient>
            </defs>
            {/* 底层 — 最浅 */}
            <path d="M8 38 L32 26 L56 38 L32 50 Z" fill="url(#sl1)" />
            {/* 中层 */}
            <path d="M8 30 L32 18 L56 30 L32 42 Z" fill="url(#sl2)" />
            {/* 顶层 — 最深 */}
            <path d="M8 22 L32 10 L56 22 L32 34 Z" fill="url(#sl3)" />
        </svg>
    )
}

export default function App() {
    onMount(() => {
        windowCtl.show()
        const splash = document.getElementById('splash')
        if (splash) {
            splash.classList.add('hide')
            setTimeout(() => splash.remove(), 350)
        }
    })

    return (
        <div class="h-full w-full flex flex-col overflow-hidden">
            <TitleBar
                productName="Store"
                productVersion="1.0.0"
                productLogo={<StoreLogo />}
                appKey="sa-store"
                productUrl="https://github.com/spark-apex/sa-store"
            />

            {/* 页面内容区 */}
            <main class="flex-1 overflow-y-auto overflow-x-hidden z-10 pb-16"
                style={{ background: 'var(--bg-deepest)' }}>
                <Switch>
                    <Match when={currentPage() === 'home'}><Home /></Match>
                    <Match when={currentPage() === 'browse'}><Browse /></Match>
                    <Match when={currentPage() === 'installed'}><Installed /></Match>
                </Switch>
            </main>

            <NavBar />
        </div>
    )
}
