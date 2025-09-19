import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

// Import translation files
import itTranslations from '../../public/locales/it/common.json';
import enTranslations from '../../public/locales/en/common.json';

const resources = {
  it: {
    common: itTranslations,
  },
  en: {
    common: enTranslations,
  },
};

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    lng: 'it', // Default language
    fallbackLng: 'en',
    debug: process.env.NODE_ENV === 'development',

    detection: {
      order: ['localStorage', 'navigator', 'htmlTag'],
      caches: ['localStorage'],
    },

    interpolation: {
      escapeValue: false, // React already escapes values
    },

    defaultNS: 'common',
    ns: ['common'],

    react: {
      useSuspense: false, // Disable suspense for SSR compatibility
    },
  });

export default i18n;