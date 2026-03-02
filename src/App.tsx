/// SA-Store 根组件
/// AuthProvider + TitleBar + BottomNav（含"我的"）
import { onMount, Switch, Match, createSignal } from 'solid-js'
import { TitleBar, windowCtl, createI18n } from '@sa/ui/desktop'
import { AuthProvider, BottomNav, ProfilePage, LoginDialog } from '@sa/ui/user'

/// i18n（暂无翻译文件，默认中文）
const i18n = createI18n('sa-store')
import Home from '@/pages/Home'
import Browse from '@/pages/Browse'
import Installed from '@/pages/Installed'

/// 星火极点品牌 Logo
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
            <path d="M8 38 L32 26 L56 38 L32 50 Z" fill="url(#sl1)" />
            <path d="M8 30 L32 18 L56 30 L32 42 Z" fill="url(#sl2)" />
            <path d="M8 22 L32 10 L56 22 L32 34 Z" fill="url(#sl3)" />
        </svg>
    )
}

/// SA-Store 业务 Tab
const STORE_TABS = [
    { key: 'home', label: '推荐', icon: 'i-carbon-home' },
    { key: 'browse', label: '浏览', icon: 'i-carbon-search' },
    { key: 'installed', label: '已安装', icon: 'i-carbon-download' },
]

export default function App() {
    const [page, setPage] = createSignal('home')
    const [showLogin, setShowLogin] = createSignal(false)

    onMount(() => {
        windowCtl.show()
        const splash = document.getElementById('splash')
        if (splash) {
            splash.classList.add('hide')
            setTimeout(() => splash.remove(), 350)
        }
    })

    return (
        <AuthProvider appKey="sa-store">
            <div class="h-full w-full flex flex-col overflow-hidden">
                <TitleBar
                    productName="Store"
                    productVersion="1.0.0"
                    productLogo={<StoreLogo />}
                    appKey="sa-store"
                    productUrl="https://github.com/spark-apex/sa-store"
                    languages={i18n.LANGUAGES}
                    currentLang={i18n.currentLang}
                    currentLangCC={i18n.currentLangCC}
                    onLangChange={i18n.setLang}
                    isLangActive={i18n.isLangActive}
                />

                {/* 页面内容区 */}
                <main class="flex-1 overflow-y-auto overflow-x-hidden z-10 pb-14"
                    style={{ background: 'var(--bg-deepest)' }}>
                    <Switch>
                        <Match when={page() === 'home'}><Home /></Match>
                        <Match when={page() === 'browse'}><Browse /></Match>
                        <Match when={page() === 'installed'}><Installed /></Match>
                        <Match when={page() === '__profile__'}><ProfilePage /></Match>
                    </Switch>
                </main>

                {/* 统一底部导航 */}
                <BottomNav
                    tabs={STORE_TABS}
                    current={page}
                    onChange={setPage}
                    onLogin={() => setShowLogin(true)}
                />
            </div>

            {/* 弹窗式登录（业务操作中触发） */}
            <LoginDialog open={showLogin()} onClose={() => setShowLogin(false)} />
        </AuthProvider>
    )
}
