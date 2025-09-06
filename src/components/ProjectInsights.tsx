import React from 'react';
import Card from './ui/Card';
import Badge from './ui/Badge';
import { ProjectInsights } from '../types';
import { ExternalLink } from 'lucide-react';

type Props = {
  insights: ProjectInsights;
};

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
  <Card className="p-4">
    <h3 className="text-sm font-semibold text-foreground mb-3">{title}</h3>
    {children}
  </Card>
);

const Row: React.FC<{ label: string; value: React.ReactNode }> = ({ label, value }) => (
  <div className="flex justify-between text-sm py-1">
    <span className="text-foreground-secondary">{label}</span>
    <span className="text-foreground">{value}</span>
  </div>
);

// Helper function to convert git URL to web URL
const getWebUrl = (gitUrl: string): string | null => {
  let url = gitUrl;
  
  // Remove .git extension if present
  if (url.endsWith('.git')) {
    url = url.slice(0, -4);
  }
  
  // Convert SSH URLs to HTTPS
  // git@github.com:user/repo.git -> https://github.com/user/repo
  const sshMatch = url.match(/^git@([^:]+):(.+)$/);
  if (sshMatch) {
    return `https://${sshMatch[1]}/${sshMatch[2]}`;
  }
  
  // If it's already an HTTPS URL, return it
  if (url.startsWith('http://') || url.startsWith('https://')) {
    return url;
  }
  
  return null;
};

const ProjectInsightsComponent: React.FC<Props> = ({ insights }) => {
  const { git_status, readme_info, ci_info, package_info, testing_info } = insights;

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
      <Section title="Git">
        <div className="space-y-2">
          <Row label="Repository" value={git_status.is_git_repo ? <Badge variant="green">Yes</Badge> : <Badge variant="gray">No</Badge>} />
          {git_status.current_branch && <Row label="Branch" value={git_status.current_branch} />}
          {typeof git_status.commit_count === 'number' && <Row label="Commits" value={git_status.commit_count} />}
          
          {/* Display remotes */}
          {git_status.remotes && git_status.remotes.length > 0 && (
            <div className="mt-3 pt-3 border-t border-border">
              <h4 className="text-xs font-medium text-foreground-secondary mb-2">Remotes</h4>
              <div className="space-y-2">
                {git_status.remotes.map((remote, index) => {
                  const webUrl = getWebUrl(remote.url);
                  return (
                    <div key={index} className="text-xs">
                      <div className="flex items-start justify-between">
                        <span className="font-medium text-foreground">{remote.name}</span>
                        {webUrl && (
                          <a 
                            href={webUrl} 
                            target="_blank" 
                            rel="noopener noreferrer"
                            className="text-primary hover:text-primary-hover flex items-center gap-1 ml-2"
                          >
                            <ExternalLink className="h-3 w-3" />
                          </a>
                        )}
                      </div>
                      <div className="text-foreground-tertiary truncate mt-0.5" title={remote.url}>
                        {remote.url}
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          )}
        </div>
      </Section>

      <Section title="README">
        <Row label="Exists" value={readme_info.exists ? <Badge variant="green">Yes</Badge> : <Badge variant="red">No</Badge>} />
        {readme_info.path && <Row label="Path" value={<span className="truncate max-w-[220px] inline-block align-bottom" title={readme_info.path}>{readme_info.path}</span>} />}
      </Section>

      <Section title="CI">
        <Row label="Configured" value={ci_info.has_ci ? <Badge variant="green">Yes</Badge> : <Badge variant="red">No</Badge>} />
        {ci_info.ci_platforms.length > 0 && (
          <div className="mt-1 flex flex-wrap gap-2">
            {ci_info.ci_platforms.map((p) => (
              <Badge key={p} variant="blue">{p}</Badge>
            ))}
          </div>
        )}
      </Section>

      <Section title="Packages">
        <div className="flex flex-wrap gap-2 text-xs">
          {package_info.has_package_json && <Badge variant="gray">package.json</Badge>}
          {package_info.has_cargo_toml && <Badge variant="gray">Cargo.toml</Badge>}
          {package_info.has_requirements_txt && <Badge variant="gray">requirements.txt</Badge>}
          {package_info.has_gemfile && <Badge variant="gray">Gemfile</Badge>}
          {package_info.has_go_mod && <Badge variant="gray">go.mod</Badge>}
        </div>
        {package_info.missing_common_files.length > 0 && (
          <div className="mt-2">
            <div className="text-xs text-foreground-secondary mb-1">Missing common files:</div>
            <div className="flex flex-wrap gap-2 text-xs">
              {package_info.missing_common_files.map((f) => (
                <Badge key={f} variant="red">{f}</Badge>
              ))}
            </div>
          </div>
        )}
      </Section>

      <Section title="Testing">
        <Row label="Has Framework" value={testing_info.has_testing_framework ? <Badge variant="green">Yes</Badge> : <Badge variant="red">No</Badge>} />
        <Row label="Test Files" value={testing_info.has_test_files ? testing_info.test_file_count : 0} />
        {typeof testing_info.source_to_test_ratio === 'number' && (
          <Row label="Source/Test Ratio" value={testing_info.source_to_test_ratio?.toFixed(2)} />
        )}
        {testing_info.testing_frameworks.length > 0 && (
          <div className="mt-1 flex flex-wrap gap-2 text-xs">
            {testing_info.testing_frameworks.map((f) => (
              <Badge key={f} variant="gray">{f}</Badge>
            ))}
          </div>
        )}
      </Section>
    </div>
  );
};

export default ProjectInsightsComponent;