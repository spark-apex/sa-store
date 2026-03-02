/// SA-Store API 服务
const API_BASE = import.meta.env?.VITE_STORE_API || 'http://localhost:3011'

/// 应用摘要
export interface AppSummary {
    id: string
    app_key: string
    name: string
    description: string | null
    logo_url: string | null
    category: string
    is_official: boolean
    total_downloads: number
    latest_version: string | null
}

/// 应用详情
export interface AppDetail {
    id: string
    app_key: string
    name: string
    description: string | null
    logo_url: string | null
    screenshots: string[]
    category: string
    is_official: boolean
    total_downloads: number
    created_at: string
}

/// 版本信息
export interface AppVersion {
    id: string
    version: string
    changelog: string | null
    download_url: string
    file_size: number
    platform: string
    status: string
    created_at: string
}

/// 获取应用列表
export async function fetchApps(): Promise<AppSummary[]> {
    const resp = await fetch(`${API_BASE}/store/apps`)
    if (!resp.ok) return []
    return resp.json()
}

/// 获取应用详情
export async function fetchApp(appKey: string): Promise<AppDetail | null> {
    const resp = await fetch(`${API_BASE}/store/apps/${appKey}`)
    if (!resp.ok) return null
    return resp.json()
}

/// 获取版本列表
export async function fetchVersions(appKey: string): Promise<AppVersion[]> {
    const resp = await fetch(`${API_BASE}/store/apps/${appKey}/versions`)
    if (!resp.ok) return []
    return resp.json()
}

/// 格式化文件大小
export function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`
}

/// 格式化下载量
export function formatDownloads(n: number): string {
    if (n < 1000) return `${n}`
    if (n < 10000) return `${(n / 1000).toFixed(1)}K`
    return `${(n / 10000).toFixed(1)}W`
}
