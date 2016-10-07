use v5.20.0;
use warnings;

use Digest::SHA1;
use Test::More;

plan skip_all => "no executable detab in cwd" unless -x 'detab';

my @tests = (qw(
  tabsample.in.txt
));

for my $testfile (map {; "corpus/$_" } @tests) {
  my $wantfile = $testfile =~ s/\.in\./\.out\./r;

  my $w_sha = Digest::SHA1->new;
  open my $w_fh, '<', $wantfile or die "can't read $wantfile: $!";
  $w_sha->addfile($w_fh);

  my $output = `./detab $testfile`;

  my $h_sha = Digest::SHA1->new;
  $h_sha->add($output);

  my $havesha = $h_sha->hexdigest;
  my $wantsha = $w_sha->hexdigest;

  is($havesha, $wantsha, "digest of output matches for $testfile");
}

done_testing;
