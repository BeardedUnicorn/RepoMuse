import React from 'react';
import Card from './ui/Card';
import Badge from './ui/Badge';
import { ProjectInsights } from '../types';

type Props = {
  insights: ProjectInsights;
};

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
  <Card className="p-4">
    <h3 className="text-sm font-semibold text-gray-900 mb-3">{title}</h3>
    {children}
  </Card>
);

const Row: React.FC<{ label: string; value: React.ReactNode }> = ({ label, value }) => (
  <div className="flex justify-between text-sm py-1">
    <span className="text-gray-600">{label}</span>
    <span className="text-gray-900">{value}</span>
  </div>
);

const ProjectInsightsComponent: React.FC<Props> = ({ insights }) => {
  const { git_status, readme_info, ci_info, package_info, testing_info } = insights;

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
      <Section title="Git">
        <Row label="Repository" value={git_status.is_git_repo ? <Badge variant="green">Yes</Badge> : <Badge variant="gray">No</Badge>} />
        {git_status.current_branch && <Row label="Branch" value={git_status.current_branch} />}
        {typeof git_status.commit_count === 'number' && <Row label="Commits" value={git_status.commit_count} />}
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
            <div className="text-xs text-gray-600 mb-1">Missing common files:</div>
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

