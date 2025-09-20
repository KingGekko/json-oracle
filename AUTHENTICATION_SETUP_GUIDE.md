# 🔐 Authentication Setup Guide for JSON Oracle API

Complete guide to set up Google, GitHub, Microsoft login with individual user dashboards using Clerk.

## 🎯 Overview

This guide will help you create:
- ✅ **Multi-provider OAuth** (Google, GitHub, Microsoft)
- ✅ **Individual user dashboards** 
- ✅ **API key management** per user
- ✅ **Secure authentication** for your JSON Oracle API

## 🏆 Recommended Solution: Clerk

**Why Clerk?**
- 🚀 **Zero-config OAuth** - Google, GitHub, Microsoft ready
- 🎨 **Beautiful UI** - Pre-built login components
- 👥 **User Management** - Individual dashboards built-in
- ⚡ **React Native** - Perfect for your Bolt UI
- 💰 **Free Tier** - 10,000 users free

## 🚀 Quick Setup (30 minutes)

### Step 1: Create Clerk Account
1. Go to [clerk.com](https://clerk.com)
2. Sign up for free account
3. Create new application
4. Choose "React" as framework

### Step 2: Configure OAuth Providers

#### Google OAuth Setup:
1. Go to Google Cloud Console
2. Create new project or select existing
3. Enable Google+ API
4. Create OAuth 2.0 credentials
5. Add authorized redirect URI: `https://your-app.clerk.accounts.dev/v1/oauth_callback`
6. Copy Client ID and Secret to Clerk dashboard

#### GitHub OAuth Setup:
1. Go to GitHub Settings → Developer settings
2. Create new OAuth App
3. Set Authorization callback URL: `https://your-app.clerk.accounts.dev/v1/oauth_callback`
4. Copy Client ID and Secret to Clerk dashboard

#### Microsoft OAuth Setup:
1. Go to Azure Portal → App registrations
2. Create new registration
3. Add redirect URI: `https://your-app.clerk.accounts.dev/v1/oauth_callback`
4. Copy Application ID and Secret to Clerk dashboard

### Step 3: Install Clerk in Your React App

```bash
npm install @clerk/clerk-react
```

### Step 4: Configure Clerk Provider

```typescript
// src/main.tsx or src/index.tsx
import { ClerkProvider } from '@clerk/clerk-react';

const PUBLISHABLE_KEY = "pk_test_your_publishable_key_here";

function App() {
  return (
    <ClerkProvider publishableKey={PUBLISHABLE_KEY}>
      <YourApp />
    </ClerkProvider>
  );
}
```

### Step 5: Create Login Components

```typescript
// src/components/Auth/LoginPage.tsx
import { SignIn, SignUp } from '@clerk/clerk-react';

export const LoginPage = () => {
  return (
    <div className="min-h-screen bg-gray-900 flex items-center justify-center">
      <div className="max-w-md w-full space-y-8">
        <div className="text-center">
          <h2 className="text-3xl font-bold text-white">
            JSON Oracle API
          </h2>
          <p className="mt-2 text-gray-400">
            Sign in to access your AI-powered data analysis dashboard
          </p>
        </div>
        
        <div className="bg-gray-800 rounded-lg p-6">
          <SignIn 
            appearance={{
              elements: {
                formButtonPrimary: 'bg-purple-600 hover:bg-purple-700',
                card: 'bg-transparent shadow-none',
                headerTitle: 'text-white',
                headerSubtitle: 'text-gray-400',
                socialButtonsBlockButton: 'border-gray-600 hover:bg-gray-700',
                socialButtonsBlockButtonText: 'text-gray-300',
                dividerLine: 'bg-gray-600',
                dividerText: 'text-gray-400',
                formFieldInput: 'bg-gray-700 border-gray-600 text-white',
                formFieldLabel: 'text-gray-300',
                footerActionLink: 'text-purple-400 hover:text-purple-300'
              }
            }}
            redirectUrl="/dashboard"
          />
        </div>
      </div>
    </div>
  );
};
```

### Step 6: Create User Dashboard

```typescript
// src/components/Dashboard/UserDashboard.tsx
import { useUser, useAuth } from '@clerk/clerk-react';
import { UserButton } from '@clerk/clerk-react';

export const UserDashboard = () => {
  const { user } = useUser();
  const { signOut } = useAuth();

  return (
    <div className="min-h-screen bg-gray-900">
      {/* Header */}
      <header className="bg-gray-800 border-b border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-4">
            <div className="flex items-center">
              <h1 className="text-xl font-bold text-white">JSON Oracle API</h1>
            </div>
            <div className="flex items-center space-x-4">
              <span className="text-gray-300">
                Welcome, {user?.firstName || user?.emailAddresses[0]?.emailAddress}
              </span>
              <UserButton afterSignOutUrl="/login" />
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        <div className="px-4 py-6 sm:px-0">
          <div className="bg-gray-800 rounded-lg p-6">
            <h2 className="text-2xl font-bold text-white mb-4">
              Your Personal Dashboard
            </h2>
            
            {/* User-specific content */}
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

### Step 7: Protect Routes

```typescript
// src/App.tsx
import { useAuth } from '@clerk/clerk-react';
import { LoginPage } from './components/Auth/LoginPage';
import { UserDashboard } from './components/Dashboard/UserDashboard';

function App() {
  const { isSignedIn, isLoaded } = useAuth();

  if (!isLoaded) {
    return <div>Loading...</div>;
  }

  return (
    <div className="App">
      {isSignedIn ? <UserDashboard /> : <LoginPage />}
    </div>
  );
}
```

## 🔧 Backend Integration

### Step 8: Update Your Rust API

```rust
// src/api/auth.rs
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn verify_clerk_token(
    headers: HeaderMap,
    State(state): State<ApiState>,
) -> Result<(), StatusCode> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    
    // Verify token with Clerk
    if verify_clerk_jwt(token).await.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(())
}

async fn verify_clerk_jwt(token: &str) -> Result<String, String> {
    // Implement Clerk JWT verification
    // This would call Clerk's API to verify the token
    // and return the user ID
    Ok("user_123".to_string())
}
```

### Step 9: User-Specific API Keys

```rust
// src/api/integration_manager.rs
impl IntegrationManager {
    pub async fn create_user_integration(
        &self,
        user_id: &str,
        request: CreateIntegrationRequest,
    ) -> Result<Integration, String> {
        let integration_id = Uuid::new_v4().to_string();
        let api_key = format!("json_oracle_{}_{}", user_id, Uuid::new_v4().to_string().replace("-", ""));
        
        let integration = Integration {
            id: integration_id.clone(),
            user_id: user_id.to_string(), // Add user association
            name: request.name,
            system_type: request.system_type,
            api_key,
            // ... rest of integration
        };

        // Store user-specific integration
        let mut integrations = self.integrations.write().await;
        integrations.insert(integration_id.clone(), integration.clone());
        
        Ok(integration)
    }

    pub async fn get_user_integrations(&self, user_id: &str) -> Vec<Integration> {
        let integrations = self.integrations.read().await;
        integrations
            .values()
            .filter(|integration| integration.user_id == user_id)
            .cloned()
            .collect()
    }
}
```

## 🎨 UI Customization

### Custom Login Page Styling

```typescript
// src/components/Auth/LoginPage.tsx
<SignIn 
  appearance={{
    baseTheme: undefined,
    elements: {
      // Custom styling to match your brand
      formButtonPrimary: 'bg-purple-600 hover:bg-purple-700 text-white',
      card: 'bg-gray-800 border border-gray-700 shadow-xl',
      headerTitle: 'text-white text-2xl font-bold',
      headerSubtitle: 'text-gray-400',
      socialButtonsBlockButton: 'border-gray-600 hover:bg-gray-700 text-gray-300',
      socialButtonsBlockButtonText: 'text-gray-300',
      formFieldInput: 'bg-gray-700 border-gray-600 text-white focus:border-purple-500',
      formFieldLabel: 'text-gray-300',
      footerActionLink: 'text-purple-400 hover:text-purple-300',
      identityPreviewText: 'text-gray-400',
      formFieldSuccessText: 'text-green-400',
      formFieldErrorText: 'text-red-400'
    },
    variables: {
      colorPrimary: '#8b5cf6',
      colorBackground: '#1f2937',
      colorText: '#ffffff',
      colorTextSecondary: '#9ca3af'
    }
  }}
  redirectUrl="/dashboard"
  signUpUrl="/sign-up"
/>
```

## 🔒 Security Features

### JWT Token Verification

```rust
// src/api/auth.rs
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub async fn verify_clerk_jwt(token: &str) -> Result<ClerkUser, String> {
    let decoding_key = DecodingKey::from_secret(get_clerk_secret().as_ref());
    let validation = Validation::new(Algorithm::RS256);
    
    match decode::<ClerkClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            // Verify token with Clerk's public key
            let user = ClerkUser {
                id: token_data.claims.sub,
                email: token_data.claims.email,
                // ... other user data
            };
            Ok(user)
        }
        Err(_) => Err("Invalid token".to_string())
    }
}

#[derive(Debug, Deserialize)]
struct ClerkClaims {
    sub: String,    // User ID
    email: String,
    iat: u64,
    exp: u64,
}
```

## 📊 User Analytics

### Track User Activity

```typescript
// src/components/Dashboard/UserDashboard.tsx
import { useAnalytics } from '@clerk/clerk-react';

export const UserDashboard = () => {
  const { track } = useAnalytics();

  const handleApiKeyGeneration = () => {
    track('api_key_generated', {
      user_id: user?.id,
      timestamp: new Date().toISOString()
    });
  };

  const handleIntegrationCreated = () => {
    track('integration_created', {
      user_id: user?.id,
      integration_type: 'webhook',
      timestamp: new Date().toISOString()
    });
  };

  // ... rest of component
};
```

## 🚀 Deployment

### Environment Variables

```bash
# .env
CLERK_PUBLISHABLE_KEY=pk_test_your_key_here
CLERK_SECRET_KEY=sk_test_your_secret_here
CLERK_WEBHOOK_SECRET=whsec_your_webhook_secret_here
```

### Docker Configuration

```dockerfile
# Dockerfile
FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .

# Build the app
RUN npm run build

EXPOSE 3000

CMD ["npm", "start"]
```

## 📈 Pricing Comparison

| Service | Free Tier | Paid Plans | Best For |
|---------|-----------|------------|----------|
| **Clerk** | 10,000 MAU | $25/month | React apps, startups |
| **Auth0** | 7,000 MAU | $23/month | Enterprise, complex needs |
| **Firebase** | 50k auth/month | Pay-per-use | Google ecosystem |
| **Supabase** | 50,000 MAU | $25/month | Full-stack apps |

## 🎯 Next Steps

1. **Choose Clerk** (recommended) or Auth0
2. **Set up OAuth providers** (Google, GitHub, Microsoft)
3. **Install and configure** authentication in your React app
4. **Update Rust API** to verify user tokens
5. **Create user-specific** API keys and integrations
6. **Deploy and test** the complete system

## 🔗 Resources

- [Clerk Documentation](https://clerk.com/docs)
- [Auth0 Documentation](https://auth0.com/docs)
- [Firebase Auth Documentation](https://firebase.google.com/docs/auth)
- [OAuth Provider Setup Guides](https://clerk.com/docs/authentication/social-connections)

---

**Your JSON Oracle API will now have professional authentication with individual user dashboards!** 🚀
