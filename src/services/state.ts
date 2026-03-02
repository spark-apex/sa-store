/// 应用状态管理
import { createSignal } from 'solid-js'

/// 当前页面
export const [currentPage, setCurrentPage] = createSignal<'home' | 'browse' | 'installed' | 'settings'>('home')
