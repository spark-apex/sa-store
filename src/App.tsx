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
        <svg width="16" height="16" viewBox="0 35 500 440" fill="none">
            <defs>
                <linearGradient id="sl1" x1="0" y1="0" x2="0.6" y2="1">
                    <stop offset="0%" stop-color="#1296DB" />
                    <stop offset="100%" stop-color="#80D8FF" />
                </linearGradient>
            </defs>
            <path d="M 256 60 L 432 162 L 432 358 L 256 456 L 80 358 L 80 162 Z"
                fill="none" stroke="url(#sl1)" stroke-width="28" stroke-linejoin="round" />
            <circle cx="256" cy="258" r="10" fill="#1296DB" />
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
