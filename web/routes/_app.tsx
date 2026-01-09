import { type PageProps } from "$fresh/server.ts";
export default function App({ Component }: PageProps) {
  return (
    <html>
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Panchangam | Cosmic Precision</title>
        <link rel="stylesheet" href="/styles.css" />
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link
          rel="preconnect"
          href="https://fonts.gstatic.com"
          crossOrigin="true"
        />
        <link
          href="https://fonts.googleapis.com/css2?family=Cinzel:wght@400;700;900&family=Inter:wght@300;400;600&family=JetBrains+Mono:wght@400;700&family=Outfit:wght@300;400;600;700&display=swap"
          rel="stylesheet"
        />
      </head>
      <body class="bg-background text-headline font-body antialiased">
        <Component />
      </body>
    </html>
  );
}
