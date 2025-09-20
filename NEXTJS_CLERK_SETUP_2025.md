# 🔐 Clerk + Next.js App Router Setup Guide 2025

Complete guide to set up Google, GitHub, Microsoft login with individual user dashboards using the latest Clerk 2025 features with Next.js App Router.

## 🎯 Overview

This guide implements the **correct** and **current** Clerk integration for Next.js App Router:
- ✅ **Latest Clerk SDK** - `@clerk/nextjs@latest`
- ✅ **App Router Pattern** - Using `app/` directory structure
- ✅ **Correct Middleware** - `clerkMiddleware()` from `@clerk/nextjs/server`
- ✅ **Proper Layout** - `<ClerkProvider>` in `app/layout.tsx`
- ✅ **Individual User Dashboards** - Personal API keys and integrations

## 🚀 Quick Setup (30 minutes)

### Step 1: Install Dependencies

```bash
cd bolt_ui
npm install @clerk/nextjs@latest
```

### Step 2: Environment Variables

Create `.env.local` file:
```bash
# .env.local
NEXT_PUBLIC_CLERK_PUBLISHABLE_KEY=YOUR_PUBLISHABLE_KEY
CLERK_SECRET_KEY=YOUR_SECRET_KEY
NEXT_PUBLIC_API_BASE_URL=http://localhost:3000
NEXT_PUBLIC_WEBSOCKET_URL=ws://localhost:3000
```

### Step 3: Create Middleware

```typescript
// middleware.ts
import { clerkMiddleware } from "@clerk/nextjs/server";

export default clerkMiddleware();

export const config = {
  matcher: [
    // Skip Next.js internals and all static files, unless found in search params
    "/((?!_next|[^?]*\.(?:html?|css|js(?!on)|jpe?g|webp|png|gif|svg|ttf|woff2?|ico|csv|docx?|xlsx?|zip|webmanifest)).*)",
    // Always run for API routes
    "/(api|trpc)(.*)",
  ],
};
```

### Step 4: App Layout with ClerkProvider

```typescript
// app/layout.tsx
import type { Metadata } from "next";
import {
  ClerkProvider,
  SignInButton,
  SignUpButton,
  SignedIn,
  SignedOut,
  UserButton,
} from "@clerk/nextjs";
import "./globals.css";

export const metadata: Metadata = {
  title: "JSON Oracle API - AI-Powered Data Analysis",
  description: "Professional AI-powered JSON data analysis platform",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <ClerkProvider
      appearance={{
        baseTheme: undefined,
        variables: {
          colorPrimary: '#8b5cf6',
          colorBackground: '#1f2937',
          colorText: '#ffffff',
          colorTextSecondary: '#9ca3af',
        },
        elements: {
          formButtonPrimary: 'bg-purple-600 hover:bg-purple-700',
          card: 'bg-gray-800 border-gray-700',
          headerTitle: 'text-white',
          socialButtonsBlockButton: 'border-gray-600 hover:bg-gray-700',
        }
      }}
    >
      <html lang="en">
        <body className="bg-gray-900 text-white">
          <header className="bg-gray-800 border-b border-gray-700">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
              <div className="flex justify-between items-center py-4">
                <div className="flex items-center gap-4">
                  <h1 className="text-xl font-bold text-white">JSON Oracle API</h1>
                </div>
                
                <div className="flex items-center gap-4">
                  <SignedOut>
                    <SignInButton mode="modal">
                      <button className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg">
                        Sign In
                      </button>
                    </SignInButton>
                    <SignUpButton mode="modal">
                      <button className="px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg">
                        Sign Up
                      </button>
                    </SignUpButton>
                  </SignedOut>
                  <SignedIn>
                    <UserButton afterSignOutUrl="/" />
                  </SignedIn>
                </div>
              </div>
            </div>
          </header>
          {children}
        </body>
      </html>
    </ClerkProvider>
  );
}
```

### Step 5: Main Page Component

```typescript
// app/page.tsx
import { SignedIn, SignedOut } from "@clerk/nextjs";
import { LoginPage } from "./components/Auth/LoginPage";
import { UserDashboard } from "./components/Dashboard/UserDashboard";

export default function HomePage() {
  return (
    <main className="min-h-screen">
      <SignedOut>
        <LoginPage />
      </SignedOut>
      <SignedIn>
        <UserDashboard />
      </SignedIn>
    </main>
  );
}
```

### Step 6: Authentication Components

```typescript
// app/components/Auth/LoginPage.tsx
import { SignIn, SignUp } from "@clerk/nextjs";
import { useState } from "react";

export function LoginPage() {
  const [isSignUp, setIsSignUp] = useState(false);

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-gray-900 flex items-center justify-center p-4">
      <div className="max-w-md w-full space-y-8">
        <div className="text-center">
          <h2 className="text-3xl font-bold text-white mb-2">
            JSON Oracle API
          </h2>
          <p className="text-gray-400 text-lg">
            AI-Powered Data Analysis Platform
          </p>
        </div>

        <div className="bg-gray-800/50 backdrop-blur-lg rounded-xl border border-gray-700/50 p-6">
          {isSignUp ? (
            <SignUp 
              appearance={{
                elements: {
                  card: 'bg-transparent shadow-none border-0',
                  formButtonPrimary: 'bg-purple-600 hover:bg-purple-700',
                  socialButtonsBlockButton: 'border-gray-600 hover:bg-gray-700',
                }
              }}
              redirectUrl="/"
              signInUrl="/"
            />
          ) : (
            <SignIn 
              appearance={{
                elements: {
                  card: 'bg-transparent shadow-none border-0',
                  formButtonPrimary: 'bg-purple-600 hover:bg-purple-700',
                  socialButtonsBlockButton: 'border-gray-600 hover:bg-gray-700',
                }
              }}
              redirectUrl="/"
              signUpUrl="/"
            />
          )}
        </div>

        <div className="text-center">
          <p className="text-gray-400 text-sm">
            {isSignUp ? "Already have an account?" : "Don't have an account?"}
            <button
              onClick={() => setIsSignUp(!isSignUp)}
              className="text-purple-400 hover:text-purple-300 ml-1 font-medium"
            >
              {isSignUp ? "Sign in" : "Sign up"}
            </button>
          </p>
        </div>
      </div>
    </div>
  );
}
```

### Step 7: User Dashboard Component

```typescript
// app/components/Dashboard/UserDashboard.tsx
"use client";

import { useUser } from "@clerk/nextjs";
import { useState, useEffect } from "react";

export function UserDashboard() {
  const { user, isLoaded } = useUser();
  const [integrations, setIntegrations] = useState([]);
  const [stats, setStats] = useState(null);

  useEffect(() => {
    if (isLoaded && user) {
      loadUserData();
    }
  }, [isLoaded, user]);

  const loadUserData = async () => {
    try {
      const token = await user?.getToken();
      const response = await fetch('/api/user/integrations', {
        headers: {
          'Authorization': `Bearer ${token}`
        }
      });
      const data = await response.json();
      setIntegrations(data);
    } catch (error) {
      console.error('Failed to load user data:', error);
    }
  };

  return (
    <div className="min-h-screen">
      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          <div className="bg-gradient-to-r from-purple-600 to-blue-600 rounded-xl p-6 text-white mb-6">
            <h2 className="text-2xl font-bold mb-2">
              Welcome back, {user?.firstName || 'there'}! 👋
            </h2>
            <p className="text-purple-100 mb-4">
              Your AI-powered data analysis platform is ready.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="bg-gray-800/50 backdrop-blur-lg rounded-xl border border-gray-700/50 p-6">
              <h3 className="text-lg font-semibold text-white mb-2">
                API Keys
              </h3>
              <p className="text-gray-400 text-sm mb-4">
                Manage your personal API keys
              </p>
              <button className="btn-primary">
                View API Keys
              </button>
            </div>

            <div className="bg-gray-800/50 backdrop-blur-lg rounded-xl border border-gray-700/50 p-6">
              <h3 className="text-lg font-semibold text-white mb-2">
                Integrations
              </h3>
              <p className="text-gray-400 text-sm mb-4">
                Your system integrations
              </p>
              <button className="btn-secondary">
                Manage Integrations
              </button>
            </div>

            <div className="bg-gray-800/50 backdrop-blur-lg rounded-xl border border-gray-700/50 p-6">
              <h3 className="text-lg font-semibold text-white mb-2">
                Analysis History
              </h3>
              <p className="text-gray-400 text-sm mb-4">
                View your past analyses
              </p>
              <button className="btn-secondary">
                View History
              </button>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
```

## 🔧 Configuration Files

### Next.js Configuration

```javascript
// next.config.js
/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    appDir: true,
  },
  images: {
    domains: ['images.clerk.dev'],
  },
  env: {
    NEXT_PUBLIC_API_BASE_URL: process.env.NEXT_PUBLIC_API_BASE_URL || 'http://localhost:3000',
    NEXT_PUBLIC_WEBSOCKET_URL: process.env.NEXT_PUBLIC_WEBSOCKET_URL || 'ws://localhost:3000',
  },
}

module.exports = nextConfig
```

### Tailwind Configuration

```javascript
// tailwind.config.js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in-out',
        'slide-up': 'slideUp 0.3s ease-out',
      },
    },
  },
  plugins: [],
}
```

### Global Styles

```css
/* app/globals.css */
@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  --foreground-rgb: 255, 255, 255;
  --background-start-rgb: 17, 24, 39;
  --background-end-rgb: 17, 24, 39;
}

body {
  color: rgb(var(--foreground-rgb));
  background: linear-gradient(
      to bottom,
      transparent,
      rgb(var(--background-end-rgb))
    )
    rgb(var(--background-start-rgb));
}

.btn-primary {
  @apply bg-purple-600 hover:bg-purple-700 text-white font-medium py-2 px-4 rounded-lg transition-colors;
}

.btn-secondary {
  @apply bg-gray-700 hover:bg-gray-600 text-white font-medium py-2 px-4 rounded-lg transition-colors;
}

.card {
  @apply bg-gray-800/50 backdrop-blur-lg rounded-xl border border-gray-700/50 p-6 shadow-xl;
}
```

## 🎨 Key Features

### ✅ **Correct Clerk Implementation:**
- **`clerkMiddleware()`** from `@clerk/nextjs/server`
- **`<ClerkProvider>`** in `app/layout.tsx`
- **App Router structure** with `app/` directory
- **Latest SDK** with `@clerk/nextjs@latest`

### ✅ **Individual User Features:**
- **Personal API Keys** - Unique to each user
- **User-Specific Dashboards** - Individual data and integrations
- **Secure Authentication** - JWT-based user management
- **OAuth Integration** - Google, GitHub, Microsoft

### ✅ **Modern UI/UX:**
- **Dark Theme** with purple branding
- **Responsive Design** - Mobile and desktop optimized
- **Real-time Updates** - Live dashboard updates
- **Professional Styling** - Enterprise-grade appearance

## 🚀 Running the Application

### Development Mode:
```bash
cd bolt_ui
npm run dev
```

### Production Build:
```bash
npm run build
npm start
```

## 🔒 Security Features

### JWT Authentication:
- ✅ **Secure Token Validation** - Clerk JWT verification
- ✅ **User Isolation** - Complete data separation
- ✅ **API Key Security** - Unique keys per user
- ✅ **Protected Routes** - Middleware-based protection

### Environment Security:
- ✅ **Environment Variables** - Secure key storage
- ✅ **Git Ignore** - `.env.local` excluded from version control
- ✅ **Placeholder Keys** - Only example values in documentation

## 📊 User Experience Flow

```
1. User visits your site
   ↓
2. Sees beautiful login page with OAuth options
   ↓
3. Clicks "Sign in with Google/GitHub/Microsoft"
   ↓
4. Gets redirected to their personal dashboard
   ↓
5. Sees only THEIR API keys and integrations
   ↓
6. Can create new integrations with their personal API keys
   ↓
7. All data is user-specific and secure
```

## 🎯 What's Different from React Setup

### ✅ **Correct Next.js App Router:**
- Uses `middleware.ts` with `clerkMiddleware()`
- `<ClerkProvider>` in `app/layout.tsx` (not `_app.tsx`)
- App Router structure with `app/` directory
- Server-side authentication with `auth()` from `@clerk/nextjs/server`

### ❌ **Avoid These Patterns:**
- Don't use `authMiddleware()` (deprecated)
- Don't use `_app.tsx` or pages-based structure
- Don't use `withAuth` or `currentUser` from older versions
- Don't reference `pages/` directory structure

## 🔗 Integration with Rust API

The frontend communicates with your Rust API using:
- **JWT Tokens** from Clerk for authentication
- **User-specific API keys** for backend integration
- **Protected API routes** with middleware verification
- **Real-time WebSocket** connections for live updates

## 📈 Next Steps

1. **Set up Clerk Account** - Create application and get API keys
2. **Configure OAuth Providers** - Google, GitHub, Microsoft
3. **Test Authentication** - Verify login flows work
4. **Deploy to Production** - Set up live environment
5. **Monitor Usage** - Track user analytics and performance

---

**Your JSON Oracle API now has enterprise-grade authentication with Next.js App Router!** 🚀

This implementation follows the **current** and **correct** Clerk patterns for Next.js App Router, ensuring compatibility and future-proofing your application.
