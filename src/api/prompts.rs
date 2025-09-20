//! Flexible prompt builder system for multi-domain AI analysis

use crate::api::domains::{Domain, AnalysisType, OutputFormat, MultiDomainAnalysisRequest, DomainRegistry, ProcessingPriority};
use serde_json::Value;
use std::collections::HashMap;

/// Advanced prompt builder that creates domain-specific prompts
pub struct PromptBuilder {
    registry: DomainRegistry,
    custom_templates: HashMap<String, String>,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            registry: DomainRegistry::new(),
            custom_templates: HashMap::new(),
        }
    }

    /// Build a complete prompt for the given request
    pub fn build_prompt(&self, request: &MultiDomainAnalysisRequest, data: &str) -> String {
        let base_prompt = if let Some(custom_prompt) = &request.prompt {
            // Use custom prompt if provided
            custom_prompt.clone()
        } else {
            // Use domain-specific template
            self.get_domain_prompt(&request.domain, &request.analysis_type)
        };

        let enhanced_prompt = self.enhance_prompt(&base_prompt, request, data);
        self.format_output(&enhanced_prompt, &request.output_format)
    }

    /// Get domain-specific prompt template
    fn get_domain_prompt(&self, domain: &Domain, analysis_type: &AnalysisType) -> String {
        self.registry
            .get_domain_prompt(domain, analysis_type)
            .unwrap_or_else(|| self.get_fallback_prompt(analysis_type))
    }

    /// Fallback prompt for unknown combinations
    fn get_fallback_prompt(&self, analysis_type: &AnalysisType) -> String {
        match analysis_type {
            AnalysisType::Prediction => {
                "You are an AI data analyst. Analyze the following data and provide predictions:

1. DATA ANALYSIS: Identify key patterns and trends
2. PREDICTIONS: Forecast future outcomes based on historical data
3. CONFIDENCE LEVELS: Assess the reliability of predictions
4. RECOMMENDATIONS: Suggest actionable next steps
5. RISK FACTORS: Identify potential risks and mitigation strategies

Provide clear, data-driven insights.".to_string()
            }
            AnalysisType::Optimization => {
                "You are an optimization specialist. Analyze the following data and provide optimization recommendations:

1. CURRENT STATE: Assess current performance and efficiency
2. OPTIMIZATION OPPORTUNITIES: Identify areas for improvement
3. RECOMMENDATIONS: Specific actions to optimize performance
4. EXPECTED OUTCOMES: Projected benefits of optimizations
5. IMPLEMENTATION PLAN: Step-by-step optimization strategy

Focus on measurable improvements and actionable recommendations.".to_string()
            }
            AnalysisType::Monitoring => {
                "You are a monitoring specialist. Analyze the following data for monitoring insights:

1. STATUS ASSESSMENT: Current state and health indicators
2. ANOMALY DETECTION: Identify unusual patterns or outliers
3. TREND ANALYSIS: Monitor changes over time
4. ALERT RECOMMENDATIONS: Suggest when to take action
5. MONITORING PLAN: Ongoing monitoring strategy

Provide clear monitoring insights and actionable alerts.".to_string()
            }
            _ => {
                "You are an AI analyst. Analyze the following data and provide insights:

1. DATA SUMMARY: Key findings and observations
2. PATTERN ANALYSIS: Identify important patterns and trends
3. INSIGHTS: Meaningful insights from the data
4. RECOMMENDATIONS: Actionable recommendations
5. NEXT STEPS: Suggested follow-up actions

Provide clear, actionable analysis.".to_string()
            }
        }
    }

    /// Enhance prompt with domain-specific context and custom instructions
    fn enhance_prompt(&self, base_prompt: &str, request: &MultiDomainAnalysisRequest, data: &str) -> String {
        let mut enhanced = base_prompt.to_string();

        // Add domain context
        enhanced.push_str(&format!("\n\nDOMAIN: {}", request.domain.as_str().to_uppercase()));
        enhanced.push_str(&format!("\nANALYSIS TYPE: {}", request.analysis_type.as_str().to_uppercase()));

        // Add custom instructions if provided
        if let Some(custom_instructions) = &request.custom_instructions {
            enhanced.push_str(&format!("\n\nCUSTOM INSTRUCTIONS: {}", custom_instructions));
        }

        // Add data context
        enhanced.push_str(&format!("\n\nDATA TO ANALYZE:\n{}", self.format_data_for_domain(&request.domain, data)));

        // Add priority context
        if let Some(priority) = &request.priority {
            enhanced.push_str(&format!("\n\nPRIORITY LEVEL: {}", format!("{:?}", priority).to_uppercase()));
            match priority {
                ProcessingPriority::Critical => {
                    enhanced.push_str("\nThis is a CRITICAL analysis. Provide immediate, actionable insights.");
                }
                ProcessingPriority::High => {
                    enhanced.push_str("\nThis is a HIGH priority analysis. Focus on the most important insights.");
                }
                _ => {}
            }
        }

        enhanced
    }

    /// Format data appropriately for different domains
    fn format_data_for_domain(&self, domain: &Domain, data: &str) -> String {
        match domain {
            Domain::Finance => self.format_finance_data(data),
            Domain::Healthcare => self.format_healthcare_data(data),
            Domain::Ecommerce => self.format_ecommerce_data(data),
            Domain::Logistics => self.format_logistics_data(data),
            _ => self.format_generic_data(data),
        }
    }

    fn format_finance_data(&self, data: &str) -> String {
        // Try to parse and structure financial data
        if let Ok(json_data) = serde_json::from_str::<Value>(data) {
            if let Some(portfolio_summary) = json_data.get("portfolio_summary") {
                format!("PORTFOLIO DATA:\n{}", serde_json::to_string_pretty(portfolio_summary).unwrap_or(data.to_string()))
            } else {
                format!("FINANCIAL DATA:\n{}", serde_json::to_string_pretty(&json_data).unwrap_or(data.to_string()))
            }
        } else {
            format!("FINANCIAL DATA:\n{}", data)
        }
    }

    fn format_healthcare_data(&self, data: &str) -> String {
        // Format healthcare data with appropriate context
        if let Ok(json_data) = serde_json::from_str::<Value>(data) {
            format!("MEDICAL DATA:\n{}", serde_json::to_string_pretty(&json_data).unwrap_or(data.to_string()))
        } else {
            format!("MEDICAL DATA:\n{}", data)
        }
    }

    fn format_ecommerce_data(&self, data: &str) -> String {
        // Format e-commerce data with business context
        if let Ok(json_data) = serde_json::from_str::<Value>(data) {
            format!("E-COMMERCE DATA:\n{}", serde_json::to_string_pretty(&json_data).unwrap_or(data.to_string()))
        } else {
            format!("E-COMMERCE DATA:\n{}", data)
        }
    }

    fn format_logistics_data(&self, data: &str) -> String {
        // Format logistics data with operational context
        if let Ok(json_data) = serde_json::from_str::<Value>(data) {
            format!("LOGISTICS DATA:\n{}", serde_json::to_string_pretty(&json_data).unwrap_or(data.to_string()))
        } else {
            format!("LOGISTICS DATA:\n{}", data)
        }
    }

    fn format_generic_data(&self, data: &str) -> String {
        // Format generic data
        if let Ok(json_data) = serde_json::from_str::<Value>(data) {
            format!("DATA:\n{}", serde_json::to_string_pretty(&json_data).unwrap_or(data.to_string()))
        } else {
            format!("DATA:\n{}", data)
        }
    }

    /// Format output based on requested format
    fn format_output(&self, prompt: &str, output_format: &Option<OutputFormat>) -> String {
        match output_format {
            Some(OutputFormat::Structured) => {
                format!("{}\n\nOUTPUT FORMAT: Please structure your response with clear sections and bullet points for easy reading.", prompt)
            }
            Some(OutputFormat::Narrative) => {
                format!("{}\n\nOUTPUT FORMAT: Please provide a narrative, story-like response that flows naturally.", prompt)
            }
            Some(OutputFormat::BulletPoints) => {
                format!("{}\n\nOUTPUT FORMAT: Please format your response as bullet points with clear, concise statements.", prompt)
            }
            Some(OutputFormat::Table) => {
                format!("{}\n\nOUTPUT FORMAT: Please format key findings in table format where appropriate.", prompt)
            }
            Some(OutputFormat::Json) => {
                format!("{}\n\nOUTPUT FORMAT: Please provide your response in JSON format with structured fields.", prompt)
            }
            Some(OutputFormat::Custom(format)) => {
                format!("{}\n\nOUTPUT FORMAT: {}", prompt, format)
            }
            None => prompt.to_string(),
        }
    }

    /// Add custom template for a specific domain/analysis type combination
    pub fn add_custom_template(&mut self, domain: Domain, analysis_type: AnalysisType, template: String) {
        let key = format!("{}:{}", domain.as_str(), analysis_type.as_str());
        self.custom_templates.insert(key, template);
    }

    /// Get supported domains
    pub fn get_supported_domains(&self) -> Vec<Domain> {
        self.registry.get_supported_domains()
    }

    /// Get supported analysis types for a domain
    pub fn get_supported_analysis_types(&self, domain: &Domain) -> Vec<AnalysisType> {
        // This would typically come from domain config
        vec![
            AnalysisType::Prediction,
            AnalysisType::Optimization,
            AnalysisType::Monitoring,
            AnalysisType::Classification,
            AnalysisType::AnomalyDetection,
            AnalysisType::TrendAnalysis,
            AnalysisType::RiskAssessment,
            AnalysisType::PerformanceAnalysis,
            AnalysisType::Custom,
        ]
    }
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for prompt building
pub mod utils {
    use super::*;

    /// Create a quick prompt for common use cases
    pub fn create_quick_prompt(domain: Domain, analysis_type: AnalysisType, data: &str) -> String {
        let builder = PromptBuilder::new();
        let request = MultiDomainAnalysisRequest {
            file_path: "inline_data".to_string(),
            prompt: None,
            model: None,
            domain,
            analysis_type,
            custom_instructions: None,
            output_format: Some(OutputFormat::Structured),
            priority: Some(ProcessingPriority::Normal),
        };
        
        builder.build_prompt(&request, data)
    }

    /// Validate that a domain/analysis type combination is supported
    pub fn validate_domain_analysis_combination(domain: &Domain, analysis_type: &AnalysisType) -> bool {
        let builder = PromptBuilder::new();
        builder.registry.get_domain_prompt(domain, analysis_type).is_some()
    }

    /// Get example prompts for different domains
    pub fn get_example_prompts() -> HashMap<String, String> {
        let mut examples = HashMap::new();
        
        examples.insert(
            "finance_prediction".to_string(),
            create_quick_prompt(Domain::Finance, AnalysisType::Prediction, "portfolio_data.json")
        );
        
        examples.insert(
            "healthcare_anomaly".to_string(),
            create_quick_prompt(Domain::Healthcare, AnalysisType::AnomalyDetection, "patient_data.json")
        );
        
        examples.insert(
            "ecommerce_optimization".to_string(),
            create_quick_prompt(Domain::Ecommerce, AnalysisType::Optimization, "sales_data.json")
        );
        
        examples
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_builder_creation() {
        let builder = PromptBuilder::new();
        assert!(!builder.get_supported_domains().is_empty());
    }

    #[test]
    fn test_prompt_building() {
        let builder = PromptBuilder::new();
        let request = MultiDomainAnalysisRequest {
            file_path: "test.json".to_string(),
            prompt: None,
            model: None,
            domain: Domain::Finance,
            analysis_type: AnalysisType::Prediction,
            custom_instructions: None,
            output_format: Some(OutputFormat::Structured),
            priority: Some(ProcessingPriority::High),
        };

        let data = r#"{"portfolio_value": 100000, "cash": 20000}"#;
        let prompt = builder.build_prompt(&request, data);
        
        assert!(prompt.contains("FINANCE"));
        assert!(prompt.contains("PREDICTION"));
        assert!(prompt.contains("PORTFOLIO DATA"));
    }

    #[test]
    fn test_custom_template() {
        let mut builder = PromptBuilder::new();
        builder.add_custom_template(
            Domain::Finance,
            AnalysisType::Custom,
            "Custom finance analysis prompt".to_string()
        );

        let request = MultiDomainAnalysisRequest {
            file_path: "test.json".to_string(),
            prompt: None,
            model: None,
            domain: Domain::Finance,
            analysis_type: AnalysisType::Custom,
            custom_instructions: None,
            output_format: None,
            priority: None,
        };

        let prompt = builder.build_prompt(&request, "test data");
        assert!(prompt.contains("Custom finance analysis prompt"));
    }

    #[test]
    fn test_quick_prompt_creation() {
        let prompt = utils::create_quick_prompt(Domain::Healthcare, AnalysisType::AnomalyDetection, "patient_data");
        assert!(prompt.contains("MEDICAL DATA"));
        assert!(prompt.contains("ANOMALY DETECTION"));
    }
}
