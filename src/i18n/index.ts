/// SA-Store i18n 翻译入口
/// 所有语言的翻译表在此合并
import zh from './zh'
import en from './en'
import zhHant from './zh-Hant'
import { ko, fr, de, es, pt, ru, ar } from './intl'

/// 翻译映射：语言 code → 翻译表
/// 不在此映射中的语言会自动回退到 zh（中文简体）
export const translations: Record<string, Record<string, string>> = {
    'zh': zh,
    'en': en,
    'zh-Hant': zhHant,
    'yue': zh,         // 粤语 UI 同简体
    'ko': ko,
    'fr': fr,
    'de': de,
    'es': es,
    'pt': pt,
    'ru': ru,
    'ar': ar,
    // 少数民族语言 → 回退中文
    'ug': zh, 'bo': zh, 'mn': zh, 'ko-CN': ko,
    'ii': zh, 'za': zh, 'dai': zh, 'lis': zh, 'mnc': zh, 'sjo': zh,
}
