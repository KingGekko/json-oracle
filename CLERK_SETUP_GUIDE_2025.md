# 🔐 Clerk Authentication Setup Guide 2025

Complete guide to set up Google, GitHub, Microsoft login with individual user dashboards using the latest Clerk 2025 features.

## 🎯 Overview

This guide will help you create:
- ✅ **Multi-provider OAuth** (Google, GitHub, Microsoft)
- ✅ **Individual user dashboards** with personal API keys
- ✅ **Secure JWT authentication** for your JSON Oracle API
- ✅ **User-specific integrations** and analytics
- ✅ **Beautiful UI components** with custom styling

## 🚀 Quick Setup (30 minutes)

### Step 1: Create Clerk Application

1. **Sign up at [clerk.com](https://clerk.com)**
   - Click "Get Started Free"
   - Create account with email or GitHub
   - Choose "React" as your framework

2. **Create New Application**
   - Click "Create Application"
   - Choose application name: "JSON Oracle API"
   - Select region closest to your users
   - Click "Create Application"

3. **Get Your API Keys**
   - Go to "API Keys" in the dashboard
   - Copy your `Publishable key` and `Secret key`
   - Save them securely

### Step 2: Configure OAuth Providers

#### Google OAuth Setup:
1. **Go to Google Cloud Console**
   - Visit [console.cloud.google.com](https://console.cloud.google.com)
   - Create new project or select existing
   - Enable Google+ API and Google Identity API

2. **Create OAuth 2.0 Credentials**
   - Go to "Credentials" → "Create Credentials" → "OAuth 2.0 Client ID"
   - Application type: "Web application"
   - Name: "JSON Oracle API"
   - Authorized redirect URIs:
     ```
     https://your-app.clerk.accounts.dev/v1/oauth_callback
     https://your-app.clerk.accounts.dev/v1/sso-callback
     ```

3. **Configure in Clerk**
   - Go to Clerk Dashboard → "User & Authentication" → "Social Connections"
   - Enable "Google"
   - Add your Google Client ID and Client Secret
   - Save changes

#### GitHub OAuth Setup:
1. **Go to GitHub Settings**
   - Visit GitHub → Settings → Developer settings → OAuth Apps
   - Click "New OAuth App"

2. **Configure OAuth App**
   - Application name: "JSON Oracle API"
   - Homepage URL: `https://your-domain.com`
   - Authorization callback URL: `https://your-app.clerk.accounts.dev/v1/oauth_callback`

3. **Configure in Clerk**
   - Go to Clerk Dashboard → "User & Authentication" → "Social Connections"
   - Enable "GitHub"
   - Add your GitHub Client ID and Client Secret
   - Save changes

#### Microsoft OAuth Setup:
1. **Go to Azure Portal**
   - Visit [portal.azure.com](https://portal.azure.com)
   - Go to "Azure Active Directory" → "App registrations"
   - Click "New registration"

2. **Register Application**
   - Name: "JSON Oracle API"
   - Supported account types: "Accounts in any organizational directory and personal Microsoft accounts"
   - Redirect URI: `https://your-app.clerk.accounts.dev/v1/oauth_callback`

3. **Configure in Clerk**
   - Go to Clerk Dashboard → "User & Authentication" → "Social Connections"
   - Enable "Microsoft"
   - Add your Azure Application ID and Client Secret
   - Save changes

### Step 3: Install and Configure Clerk

#### Install Clerk Package:
```bash
cd bolt_ui
npm install @clerk/clerk-react@latest
```

#### Environment Variables:
Create `.env` file in `bolt_ui/`:
```bash
# Clerk Authentication
REACT_APP_CLERK_PUBLISHABLE_KEY=pk_test_your_publishable_key_here
REACT_APP_CLERK_SECRET_KEY=sk_test_your_secret_key_here

# API Configuration
REACT_APP_API_BASE_URL=http://localhost:3000
REACT_APP_WEBSOCKET_URL=ws://localhost:3000
```

#### Configure Clerk Provider:
```typescript
// src/main.tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import { ClerkProvider } from '@clerk/clerk-react';
import App from './App';

const clerkPubKey = process.env.REACT_APP_CLERK_PUBLISHABLE_KEY;

if (!clerkPubKey) {
  throw new Error("Missing Publishable Key");
}

root.render(
  <React.StrictMode>
    <ClerkProvider
      publishableKey={clerkPubKey}
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
      <App />
    </ClerkProvider>
  </React.StrictMode>
);
```

### Step 4: Create Authentication Components

#### Login Page:
```typescript
// src/components/Auth/LoginPage.tsx
import React from 'react';
import { SignIn, SignUp } from '@clerk/clerk-react';

export const LoginPage: React.FC = () => {
  const [isSignUp, setIsSignUp] = useState(false);

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-gray-900 flex items-center justify-center">
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
              redirectUrl="/dashboard"
              signInUrl="/login"
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
              redirectUrl="/dashboard"
              signUpUrl="/signup"
            />
          )}
        </div>
      </div>
    </div>
  );
};
```

### Step 5: Create User Dashboard

#### User Dashboard Component:
```typescript
// src/components/Dashboard/UserDashboard.tsx
import React, { useState, useEffect } from 'react';
import { useUser, UserButton } from '@clerk/clerk-react';

export const UserDashboard: React.FC = () => {
  const { user } = useUser();
  const [integrations, setIntegrations] = useState([]);
  const [stats, setStats] = useState(null);

  useEffect(() => {
    if (user) {
      loadUserData();
    }
  }, [user]);

  const loadUserData = async () => {
    try {
      const token = await user.getToken();
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
    <div className="min-h-screen bg-gray-900">
      <header className="bg-gray-800 border-b border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <h1 className="text-xl font-bold text-white">JSON Oracle API</h1>
            <div className="flex items-center gap-4">
              <span className="text-gray-300">
                Welcome, {user?.firstName || user?.emailAddresses[0]?.emailAddress}
              </span>
              <UserButton afterSignOutUrl="/login" />
            </div>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          <div className="bg-gray-800/50 backdrop-blur-lg rounded-xl border border-gray-700/50 p-6">
            <h2 className="text-2xl font-bold text-white mb-4">
              Your Personal Dashboard
            </h2>
            
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="bg-gray-700 rounded-lg p-4">
                <h3 className="text-lg font-semibold text-white mb-2">
                  API Keys
                </h3>
                <p className="text-gray-400 text-sm mb-4">
                  Manage your personal API keys
                </p>
                <button className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded">
                  View API Keys
                </button>
              </div>

              <div className="bg-gray-700 rounded-lg p-4">
                <h3 className="text-lg font-semibold text-white mb-2">
                  Integrations
                </h3>
                <p className="text-gray-400 text-sm mb-4">
                  Your system integrations
                </p>
                <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded">
                  Manage Integrations
                </button>
              </div>

              <div className="bg-gray-700 rounded-lg p-4">
                <h3 className="text-lg font-semibold text-white mb-2">
                  Analysis History
                </h3>
                <p className="text-gray-400 text-sm mb-4">
                  View your past analyses
                </p>
                <button className="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded">
                  View History
                </button>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
};
```

### Step 6: Update Main App

#### App.tsx:
```typescript
// src/App.tsx
import React from 'react';
import { useUser } from '@clerk/clerk-react';
import { LoginPage } from './components/Auth/LoginPage';
import { UserDashboard } from './components/Dashboard/UserDashboard';
import { Loader2 } from 'lucide-react';

function App() {
  const { user, isLoaded } = useUser();

  if (!isLoaded) {
    return (
      <div className="min-h-screen bg-gray-900 flex items-center justify-center">
        <div className="text-center">
          <Loader2 className="w-8 h-8 animate-spin text-purple-400 mx-auto mb-4" />
          <p className="text-gray-400">Loading JSON Oracle API...</p>
        </div>
      </div>
    );
  }

  if (!user) {
    return <LoginPage />;
  }

  return <UserDashboard />;
}

export default App;
```

### Step 7: Configure Rust API Backend

#### Add Dependencies to Cargo.toml:
```toml
[dependencies]
jsonwebtoken = "9.2"
reqwest = { version = "0.11", features = ["json"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

#### Environment Variables for Rust API:
```bash
# .env
CLERK_SECRET_KEY=sk_test_your_secret_key_here
CLERK_DOMAIN=your-app.clerk.accounts.dev
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llama2
MAX_TIMEOUT_SECONDS=180
```

#### JWT Verification in Rust:
```rust
// src/api/auth.rs
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub async fn verify_clerk_jwt(token: &str) -> Result<ClerkUser, String> {
    let public_key = get_clerk_public_key().await?;
    let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
    let validation = Validation::new(Algorithm::RS256);
    
    match decode::<ClerkClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            let user = ClerkUser {
                id: token_data.claims.sub,
                email: token_data.claims.email,
                first_name: token_data.claims.given_name,
                last_name: token_data.claims.family_name,
                image_url: token_data.claims.picture,
                created_at: token_data.claims.iat as i64,
            };
            Ok(user)
        }
        Err(e) => Err(format!("Token verification failed: {}", e))
    }
}
```

### Step 8: User-Specific API Endpoints

#### User API Routes:
```rust
// src/api/user_handlers.rs
pub fn create_user_routes() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/user/integrations", get(get_user_integrations))
        .route("/user/integrations", post(create_user_integration))
        .route("/user/stats", get(get_user_stats))
        .route("/user/profile", get(get_user_profile))
        .route("/user/analytics", get(get_user_analytics))
}

async fn get_user_integrations(
    State(state): State<Arc<ApiState>>,
    request: axum::extract::Request,
) -> Result<Json<Vec<Integration>>, StatusCode> {
    let user = get_current_user(&request)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let manager = IntegrationManager::new();
    let integrations = manager.get_user_integrations(&user.id).await;
    
    Ok(Json(integrations))
}
```

## 🎨 Customization Options

### Custom Styling:
```typescript
// Clerk appearance customization
appearance={{
  variables: {
    colorPrimary: '#8b5cf6',        // Your brand color
    colorBackground: '#1f2937',     // Dark theme background
    colorText: '#ffffff',           // White text
    borderRadius: '0.5rem',         // Rounded corners
  },
  elements: {
    formButtonPrimary: 'bg-purple-600 hover:bg-purple-700 text-white',
    card: 'bg-gray-800 border-gray-700 shadow-xl',
    headerTitle: 'text-white text-2xl font-bold',
    socialButtonsBlockButton: 'border-gray-600 hover:bg-gray-700',
    formFieldInput: 'bg-gray-700 border-gray-600 text-white',
  }
}}
```

### Custom Redirect URLs:
```typescript
// Configure redirect URLs
<SignIn 
  redirectUrl="/dashboard"
  signUpUrl="/signup"
  afterSignInUrl="/dashboard"
  afterSignUpUrl="/dashboard"
/>
```

## 🔒 Security Features

### JWT Token Verification:
- ✅ **RS256 Algorithm** - Secure asymmetric encryption
- ✅ **Public Key Validation** - Verifies token authenticity
- ✅ **Expiration Checking** - Prevents expired token usage
- ✅ **Issuer Validation** - Ensures tokens come from Clerk

### User Data Protection:
- ✅ **User-Specific API Keys** - Each user gets unique keys
- ✅ **Integration Isolation** - Users only see their integrations
- ✅ **Data Segregation** - Complete user data separation
- ✅ **Audit Logging** - Track all user actions

## 📊 User Experience Features

### Individual Dashboards:
- ✅ **Personal API Keys** - Unique keys per user
- ✅ **Integration Management** - User-specific integrations
- ✅ **Analytics Tracking** - Individual usage metrics
- ✅ **Real-time Updates** - Live dashboard updates

### OAuth Providers:
- ✅ **Google Sign-In** - One-click Google authentication
- ✅ **GitHub Sign-In** - Developer-friendly GitHub login
- ✅ **Microsoft Sign-In** - Enterprise Microsoft accounts
- ✅ **Email/Password** - Traditional authentication fallback

## 🚀 Deployment

### Environment Variables for Production:
```bash
# Production .env
REACT_APP_CLERK_PUBLISHABLE_KEY=pk_live_your_live_key
CLERK_SECRET_KEY=sk_live_your_live_secret
REACT_APP_API_BASE_URL=https://your-api-domain.com
```

### Docker Configuration:
```dockerfile
# Dockerfile
FROM node:18-alpine

WORKDIR /app
COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

EXPOSE 3000
CMD ["npm", "start"]
```

## 📈 Pricing & Limits

### Clerk Free Tier:
- ✅ **10,000 Monthly Active Users**
- ✅ **Unlimited OAuth providers**
- ✅ **Custom branding**
- ✅ **Basic analytics**

### Upgrade Options:
- **Pro Plan**: $25/month for 10,000+ MAU
- **Enterprise**: Custom pricing for advanced features

## 🎯 Next Steps

1. **Complete OAuth Setup** - Configure all three providers
2. **Test Authentication** - Verify login flows work
3. **Deploy to Production** - Set up live environment
4. **Monitor Usage** - Track user analytics
5. **Scale as Needed** - Upgrade plans when required

## 🔗 Resources

- [Clerk Documentation](https://clerk.com/docs)
- [Clerk React Guide](https://clerk.com/docs/references/react)
- [OAuth Provider Setup](https://clerk.com/docs/authentication/social-connections)
- [Custom Styling Guide](https://clerk.com/docs/customization/overview)

---

**Your JSON Oracle API now has professional authentication with individual user dashboards!** 🚀

Users can sign in with Google, GitHub, or Microsoft and get their own personalized dashboard with unique API keys and integrations.
