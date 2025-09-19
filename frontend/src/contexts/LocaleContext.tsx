'use client';

import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { useRouter } from 'next/navigation';

type Locale = 'it' | 'en';

interface LocaleContextType {
  locale: Locale;
  setLocale: (locale: Locale) => void;
  t: (key: string, options?: { [key: string]: any }) => string;
}

const LocaleContext = createContext<LocaleContextType | undefined>(undefined);

interface LocaleProviderProps {
  children: ReactNode;
}

// Translation data will be loaded dynamically
let translations: { [locale: string]: { [key: string]: any } } = {};

export function LocaleProvider({ children }: LocaleProviderProps) {
  const [locale, setLocaleState] = useState<Locale>('it'); // Default to Italian
  const [translationsLoaded, setTranslationsLoaded] = useState(false);
  const router = useRouter();

  // Load translations
  useEffect(() => {
    const loadTranslations = async () => {
      try {
        const [itTranslations, enTranslations] = await Promise.all([
          fetch('/locales/it/common.json').then(res => res.json()),
          fetch('/locales/en/common.json').then(res => res.json()),
        ]);

        translations = {
          it: itTranslations,
          en: enTranslations,
        };

        setTranslationsLoaded(true);
      } catch (error) {
        console.error('Failed to load translations:', error);
        setTranslationsLoaded(true); // Continue even if loading fails
      }
    };

    loadTranslations();
  }, []);

  // Load locale from localStorage on mount
  useEffect(() => {
    try {
      const savedLocale = localStorage.getItem('locale') as Locale;
      if (savedLocale && (savedLocale === 'it' || savedLocale === 'en')) {
        setLocaleState(savedLocale);
      }
    } catch (error) {
      console.error('Failed to load locale from localStorage:', error);
    }
  }, []);

  // Save locale to localStorage when it changes
  const setLocale = (newLocale: Locale) => {
    setLocaleState(newLocale);
    try {
      localStorage.setItem('locale', newLocale);
    } catch (error) {
      console.error('Failed to save locale to localStorage:', error);
    }
  };

  // Translation function
  const t = (key: string, options: { [key: string]: any } = {}): string => {
    if (!translationsLoaded) {
      return key; // Return key if translations not loaded yet
    }

    const keys = key.split('.');
    let value: any = translations[locale];

    // Navigate through nested keys
    for (const k of keys) {
      if (value && typeof value === 'object' && k in value) {
        value = value[k];
      } else {
        // Fallback to English if key not found in current locale
        value = translations.en;
        for (const fallbackKey of keys) {
          if (value && typeof value === 'object' && fallbackKey in value) {
            value = value[fallbackKey];
          } else {
            return key; // Return key if not found in fallback either
          }
        }
        break;
      }
    }

    if (typeof value !== 'string') {
      return key; // Return key if final value is not a string
    }

    // Simple interpolation for {{variable}} patterns
    let result = value;
    Object.keys(options).forEach(optionKey => {
      const placeholder = `{{${optionKey}}}`;
      result = result.replace(new RegExp(placeholder, 'g'), String(options[optionKey]));
    });

    return result;
  };

  const value: LocaleContextType = {
    locale,
    setLocale,
    t,
  };

  return (
    <LocaleContext.Provider value={value}>
      {children}
    </LocaleContext.Provider>
  );
}

export function useLocale() {
  const context = useContext(LocaleContext);
  if (context === undefined) {
    throw new Error('useLocale must be used within a LocaleProvider');
  }
  return context;
}

export type { Locale };