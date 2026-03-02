/// 底部导航栏
import { currentPage, setCurrentPage } from '@/services/state'

const tabs = [
    { key: 'home' as const, icon: 'i-carbon-home', label: '推荐' },
    { key: 'browse' as const, icon: 'i-carbon-search', label: '浏览' },
    { key: 'installed' as const, icon: 'i-carbon-download', label: '已安装' },
]

export default function NavBar() {
    return (
        <nav class="fixed bottom-0 left-0 right-0 z-50 flex"
            style={{
                background: 'var(--bg-panel)',
                'border-top': '1px solid var(--border)',
                'backdrop-filter': 'blur(16px)',
            }}>
            {tabs.map((t) => (
                <button
                    class="flex-1 flex flex-col items-center gap-1 py-2.5 border-none bg-transparent cursor-pointer transition-all duration-200"
                    style={{
                        color: currentPage() === t.key ? 'var(--brand)' : 'var(--text-ghost)',
                        'font-size': '10px',
                        'font-family': 'inherit',
                    }}
                    onClick={() => setCurrentPage(t.key)}>
                    <div class={`${t.icon} text-4.5`} />
                    <span style={{ 'font-weight': currentPage() === t.key ? '600' : '400' }}>{t.label}</span>
                </button>
            ))}
        </nav>
    )
}
