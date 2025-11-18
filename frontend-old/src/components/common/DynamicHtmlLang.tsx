'use client';

import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';

export default function DynamicHtmlLang() {
  const { i18n } = useTranslation();

  useEffect(() => {
    // Update the document's lang attribute when language changes
    if (typeof document !== 'undefined') {
      document.documentElement.lang = i18n.language;
    }
  }, [i18n.language]);

  return null;
}