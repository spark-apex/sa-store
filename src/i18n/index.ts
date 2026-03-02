/// SA-Store i18n 翻译入口
import zh from './zh'
import en from './en'
import zhHant from './zh-Hant'
import { ko, fr, de, es, pt, ru, ar } from './intl'

export const translations: Record<string, Record<string, string>> = {
    'zh': zh,
    'en': en,
    'zh-Hant': zhHant,
    'yue': zh,
    'ko': ko, 'ko-CN': ko,
    'fr': fr, 'de': de, 'es': es, 'pt': pt,
    'ru': ru, 'ar': ar,
    // 少数民族语言（导航/按钮用原语言，产品文本回退中文）
    'ug': zh, 'bo': zh, 'mn': zh,
}
