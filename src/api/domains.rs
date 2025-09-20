//! Domain-specific types and configurations for multi-domain API support

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported domains for AI analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Domain {
    Finance,
    Healthcare,
    Ecommerce,
    Logistics,
    Manufacturing,
    RealEstate,
    Education,
    Environmental,
    Generic,
}

impl Domain {
    pub fn as_str(&self) -> &'static str {
        match self {
            Domain::Finance => "finance",
            Domain::Healthcare => "healthcare",
            Domain::Ecommerce => "ecommerce",
            Domain::Logistics => "logistics",
            Domain::Manufacturing => "manufacturing",
            Domain::RealEstate => "realestate",
            Domain::Education => "education",
            Domain::Environmental => "environmental",
            Domain::Generic => "generic",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "finance" => Some(Domain::Finance),
            "healthcare" => Some(Domain::Healthcare),
            "ecommerce" => Some(Domain::Ecommerce),
            "logistics" => Some(Domain::Logistics),
            "manufacturing" => Some(Domain::Manufacturing),
            "realestate" | "real_estate" => Some(Domain::RealEstate),
            "education" => Some(Domain::Education),
            "environmental" => Some(Domain::Environmental),
            "generic" => Some(Domain::Generic),
            _ => None,
        }
    }
}

/// Analysis types available across domains
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum AnalysisType {
    Prediction,
    Optimization,
    Monitoring,
    Classification,
    AnomalyDetection,
    TrendAnalysis,
    RiskAssessment,
    PerformanceAnalysis,
    Custom,
}

impl AnalysisType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnalysisType::Prediction => "prediction",
            AnalysisType::Optimization => "optimization",
            AnalysisType::Monitoring => "monitoring",
            AnalysisType::Classification => "classification",
            AnalysisType::AnomalyDetection => "anomaly_detection",
            AnalysisType::TrendAnalysis => "trend_analysis",
            AnalysisType::RiskAssessment => "risk_assessment",
            AnalysisType::PerformanceAnalysis => "performance_analysis",
            AnalysisType::Custom => "custom",
        }
    }
}

/// Enhanced request structure for multi-domain support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiDomainAnalysisRequest {
    pub file_path: String,
    pub prompt: Option<String>,
    pub model: Option<String>,
    pub domain: Domain,
    pub analysis_type: AnalysisType,
    pub custom_instructions: Option<String>,
    pub output_format: Option<OutputFormat>,
    pub priority: Option<ProcessingPriority>,
}

/// Output format preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Structured,
    Narrative,
    BulletPoints,
    Table,
    Json,
    Custom(String),
}

/// Processing priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessingPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Domain-specific configuration
#[derive(Debug, Clone)]
pub struct DomainConfig {
    pub name: String,
    pub default_prompts: HashMap<AnalysisType, String>,
    pub data_processors: Vec<String>,
    pub supported_models: Vec<String>,
    pub max_timeout_seconds: u64,
}

impl DomainConfig {
    pub fn finance() -> Self {
        let mut prompts = HashMap::new();
        prompts.insert(
            AnalysisType::Prediction,
            "You are an Elite quantitative trading analyst specializing in algorithmic trading and portfolio optimization.

ANALYZE THE FOLLOWING FINANCIAL DATA AND PROVIDE SPECIFIC, ACTIONABLE TRADING RECOMMENDATIONS:

REQUIRED OUTPUT FORMAT:
1. PORTFOLIO STATUS: Current portfolio value, positions, and cash position
2. MARKET OPPORTUNITIES: Specific BUY/SELL/HOLD recommendations with target prices
3. RISK ASSESSMENT: Risk level analysis and management strategies
4. TRADING ACTIONS: Prioritized list of actions with execution timing
5. PORTFOLIO OPTIMIZATION: Rebalancing and allocation suggestions

Focus on profit maximization and risk management.".to_string()
        );
        
        prompts.insert(
            AnalysisType::RiskAssessment,
            "You are a financial risk analyst. Analyze the following portfolio data and assess:

1. PORTFOLIO RISK METRICS: VaR, Sharpe ratio, volatility analysis
2. CONCENTRATION RISKS: Position sizing and diversification analysis
3. MARKET RISKS: Sector exposure and correlation analysis
4. RISK MITIGATION: Specific recommendations to reduce risk
5. STRESS TESTING: Scenario analysis and worst-case projections

Provide specific, actionable risk management recommendations.".to_string()
        );

        Self {
            name: "Finance".to_string(),
            default_prompts: prompts,
            data_processors: vec!["portfolio_processor".to_string(), "market_data_processor".to_string()],
            supported_models: vec!["llama2".to_string(), "codellama".to_string(), "mistral".to_string()],
            max_timeout_seconds: 120,
        }
    }

    pub fn healthcare() -> Self {
        let mut prompts = HashMap::new();
        prompts.insert(
            AnalysisType::Prediction,
            "You are a medical AI assistant specializing in clinical data analysis.

ANALYZE THE FOLLOWING MEDICAL DATA AND PROVIDE CLINICAL INSIGHTS:

REQUIRED OUTPUT FORMAT:
1. PATIENT STATUS: Current health metrics and vital signs analysis
2. RISK ASSESSMENT: Health risk factors and early warning indicators
3. TREATMENT RECOMMENDATIONS: Evidence-based treatment suggestions
4. MONITORING PLAN: Follow-up requirements and monitoring schedule
5. CLINICAL NOTES: Summary of findings and recommendations

IMPORTANT: This is for informational purposes only. Always consult with qualified healthcare professionals.".to_string()
        );

        prompts.insert(
            AnalysisType::AnomalyDetection,
            "You are a medical AI specialist focused on anomaly detection in patient data.

ANALYZE THE FOLLOWING MEDICAL DATA FOR ABNORMAL PATTERNS:

1. VITAL SIGNS ANALYSIS: Identify unusual patterns in heart rate, blood pressure, temperature
2. LAB VALUES: Detect abnormal laboratory results and trends
3. SYMPTOM PATTERNS: Identify concerning symptom combinations
4. RISK INDICATORS: Highlight high-risk conditions requiring immediate attention
5. ALERT RECOMMENDATIONS: Specific actions to take based on findings

Provide clear, prioritized alerts for healthcare professionals.".to_string()
        );

        Self {
            name: "Healthcare".to_string(),
            default_prompts: prompts,
            data_processors: vec!["patient_data_processor".to_string(), "lab_results_processor".to_string()],
            supported_models: vec!["llama2".to_string(), "medllama".to_string()],
            max_timeout_seconds: 90,
        }
    }

    pub fn ecommerce() -> Self {
        let mut prompts = HashMap::new();
        prompts.insert(
            AnalysisType::Optimization,
            "You are an e-commerce optimization specialist with expertise in data-driven business decisions.

ANALYZE THE FOLLOWING E-COMMERCE DATA AND PROVIDE OPTIMIZATION RECOMMENDATIONS:

REQUIRED OUTPUT FORMAT:
1. BUSINESS PERFORMANCE: Key metrics analysis (sales, conversion, retention)
2. CUSTOMER INSIGHTS: Behavior patterns and segmentation analysis
3. INVENTORY OPTIMIZATION: Stock level recommendations and demand forecasting
4. PRICING STRATEGY: Dynamic pricing suggestions and competitive analysis
5. MARKETING OPTIMIZATION: Campaign performance and channel recommendations

Focus on revenue growth and operational efficiency.".to_string()
        );

        prompts.insert(
            AnalysisType::Prediction,
            "You are an e-commerce data scientist specializing in demand forecasting and customer behavior prediction.

ANALYZE THE FOLLOWING E-COMMERCE DATA AND PROVIDE PREDICTIONS:

1. SALES FORECASTING: Predict future sales trends and seasonal patterns
2. CUSTOMER LIFETIME VALUE: Estimate CLV and retention predictions
3. INVENTORY DEMAND: Forecast product demand and stock requirements
4. MARKETING ROI: Predict campaign performance and customer acquisition costs
5. BUSINESS GROWTH: Project growth trajectories and scaling recommendations

Provide specific, actionable predictions with confidence intervals.".to_string()
        );

        Self {
            name: "E-commerce".to_string(),
            default_prompts: prompts,
            data_processors: vec!["sales_data_processor".to_string(), "customer_data_processor".to_string()],
            supported_models: vec!["llama2".to_string(), "mistral".to_string()],
            max_timeout_seconds: 60,
        }
    }

    pub fn logistics() -> Self {
        let mut prompts = HashMap::new();
        prompts.insert(
            AnalysisType::Optimization,
            "You are a logistics optimization expert specializing in supply chain and transportation efficiency.

ANALYZE THE FOLLOWING LOGISTICS DATA AND PROVIDE OPTIMIZATION RECOMMENDATIONS:

REQUIRED OUTPUT FORMAT:
1. ROUTE OPTIMIZATION: Delivery route analysis and efficiency improvements
2. INVENTORY MANAGEMENT: Stock level optimization and warehouse efficiency
3. TRANSPORTATION ANALYSIS: Fleet utilization and cost optimization
4. SUPPLY CHAIN RISKS: Identify bottlenecks and supply chain vulnerabilities
5. PERFORMANCE METRICS: KPIs analysis and improvement recommendations

Focus on cost reduction and operational efficiency.".to_string()
        );

        Self {
            name: "Logistics".to_string(),
            default_prompts: prompts,
            data_processors: vec!["route_data_processor".to_string(), "inventory_processor".to_string()],
            supported_models: vec!["llama2".to_string(), "codellama".to_string()],
            max_timeout_seconds: 90,
        }
    }

    pub fn get_config(domain: &Domain) -> Self {
        match domain {
            Domain::Finance => DomainConfig::finance(),
            Domain::Healthcare => DomainConfig::healthcare(),
            Domain::Ecommerce => DomainConfig::ecommerce(),
            Domain::Logistics => DomainConfig::logistics(),
            _ => DomainConfig::generic(),
        }
    }

    pub fn generic() -> Self {
        let mut prompts = HashMap::new();
        prompts.insert(
            AnalysisType::Prediction,
            "You are an AI data analyst specializing in pattern recognition and predictive insights.

ANALYZE THE FOLLOWING DATA AND PROVIDE INSIGHTS:

1. DATA SUMMARY: Key findings and patterns in the dataset
2. TREND ANALYSIS: Identify trends and patterns over time
3. PREDICTIONS: Future projections based on historical data
4. RECOMMENDATIONS: Actionable insights and next steps
5. RISK FACTORS: Potential issues and mitigation strategies

Provide clear, data-driven insights and recommendations.".to_string()
        );

        Self {
            name: "Generic".to_string(),
            default_prompts: prompts,
            data_processors: vec!["generic_processor".to_string()],
            supported_models: vec!["llama2".to_string(), "mistral".to_string()],
            max_timeout_seconds: 60,
        }
    }
}

/// Domain registry for managing all supported domains
pub struct DomainRegistry {
    configs: HashMap<Domain, DomainConfig>,
}

impl DomainRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            configs: HashMap::new(),
        };
        
        // Register all domains
        registry.register_domain(Domain::Finance);
        registry.register_domain(Domain::Healthcare);
        registry.register_domain(Domain::Ecommerce);
        registry.register_domain(Domain::Logistics);
        registry.register_domain(Domain::Manufacturing);
        registry.register_domain(Domain::RealEstate);
        registry.register_domain(Domain::Education);
        registry.register_domain(Domain::Environmental);
        registry.register_domain(Domain::Generic);
        
        registry
    }

    fn register_domain(&mut self, domain: Domain) {
        let config = DomainConfig::get_config(&domain);
        self.configs.insert(domain, config);
    }

    pub fn get_config(&self, domain: &Domain) -> Option<&DomainConfig> {
        self.configs.get(domain)
    }

    pub fn get_supported_domains(&self) -> Vec<Domain> {
        self.configs.keys().cloned().collect()
    }

    pub fn get_domain_prompt(&self, domain: &Domain, analysis_type: &AnalysisType) -> Option<String> {
        self.configs
            .get(domain)
            .and_then(|config| config.default_prompts.get(analysis_type))
            .cloned()
    }
}

impl Default for DomainRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_parsing() {
        assert_eq!(Domain::from_str("finance"), Some(Domain::Finance));
        assert_eq!(Domain::from_str("HEALTHCARE"), Some(Domain::Healthcare));
        assert_eq!(Domain::from_str("invalid"), None);
    }

    #[test]
    fn test_domain_config() {
        let registry = DomainRegistry::new();
        let config = registry.get_config(&Domain::Finance).unwrap();
        assert_eq!(config.name, "Finance");
        assert!(config.default_prompts.contains_key(&AnalysisType::Prediction));
    }

    #[test]
    fn test_request_serialization() {
        let request = MultiDomainAnalysisRequest {
            file_path: "data.json".to_string(),
            prompt: None,
            model: Some("llama2".to_string()),
            domain: Domain::Healthcare,
            analysis_type: AnalysisType::AnomalyDetection,
            custom_instructions: None,
            output_format: Some(OutputFormat::Structured),
            priority: Some(ProcessingPriority::High),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: MultiDomainAnalysisRequest = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.domain, Domain::Healthcare);
        assert_eq!(deserialized.analysis_type, AnalysisType::AnomalyDetection);
    }
}
