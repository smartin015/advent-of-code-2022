#!/usr/bin/perl
use warnings;
use strict;
use Data::Dumper;

my @cwd = ();
my %dirmap = ();
my $dirmap = \%dirmap;

my $cmd = "";

sub set_hash { # dirmap, cwd, dir|size, name
  my $ref = $_[0]; # Grabs reference
  my $cw = $_[1]; 
  my @cw = @$cw; # makes copy
  
  # print("CW @$_[1]\n");
  while (scalar(@cw) > 0) {
    my $c = shift(@cw);
    print("Handling $c\n");
    if (!exists($ref->{$c})) {
      print("Adding hashref at $c\n");
      my %h = ();
      $ref->{$c} = \%h;
    }
    $ref = $ref->{$c};
  }
  
  my $key = $_[3];
  if ($_[2] eq "dir") {
    print("Assign hashref at $key\n");
    print Dumper($ref, $key);
    my %h = ();
    $ref->{$key} = \%h;
  } else {
    my $val = int($_[2]);
    $ref->{$key} = $val;
  }
}

sub total_size {
  my $ref = $_[0];
  my $tot = 0;
  for my $key (keys %$ref) {
    if (ref($ref->{$key}) eq 'HASH') {
      $tot += total_size($ref->{$key}, )
    } else {
      $tot += $ref->{$key};
    }
  }
  return $tot;
}

#set_hash($dirmap, \@cwd, "dir", "a");
#@cwd = ('a');
#set_hash($dirmap, \@cwd, "12345", "b");
#@cwd = ('c', 'd');
#set_hash($dirmap, \@cwd, "12345", "e");

for (<STDIN>) {
  print(">: $_");
  my @split = split(' ', $_);
  if ($split[0] eq '$') {
    $cmd = $split[1];
  }

  if ($cmd eq 'cd') {
    my $p = $split[2];
    if (substr($p, 0, 0) eq '/') {
      @cwd = ();
    } elsif ($p eq '..') {
      pop(@cwd);
    } else {
      push(@cwd, split('/', $p));
    }
    print("CWD now /" . join('/', @cwd) . "\n");
  } elsif ($cmd eq 'ls' && $split[0] ne '$') {
    set_hash($dirmap, \@cwd, $split[0], $split[1]);
  }
}

# Traverse all directories and return those with a size of at most 100000
sub walk {
  my $ref = $_[0];
  my $ret = 0;
  for my $key (keys %$ref) {
    if (ref($ref->{$key}) eq 'HASH') {
      my $tot = total_size($ref->{$key});
      if ($tot < 100000) {
        $ret += $tot;
      }
      $ret += walk($ref->{$key});
    }
  }
  return $ret;
}

sub smallest_over_sz { # \%ref, $size
  my $ref = $_[0];
  my $thresh = $_[1];
  my $ret = 1000000000;
  for my $key (keys %$ref) {
    if (ref($ref->{$key}) eq 'HASH') {
      # search for optimal subdirs
      my $tot = smallest_over_sz($ref->{$key}, $thresh);
      print("Got recursive tot option for $key: $tot\n");
      if ($ret == -1 || $tot > $thresh && $tot < $ret) {
        $ret = $tot;
        print("Ret now $ret ($tot > $thresh)\n");
      }

      # also compare size of neighbor dirs
      $tot = total_size($ref->{$key});
      print("Got local tot option for $key: $tot\n");
      if ($tot > $thresh && $tot < $ret) {
        $ret = $tot;
        print("Ret now $ret\n");
      }

    }
  }
  return $ret;
}

# print(Dumper($dirmap));
print("\n");
# print(total_size($dirmap->{'a'}->{'e'}));
my $want_unused = 30000000;
my $fs_size = 70000000;
my $fs_unused = $fs_size - total_size($dirmap);
my $thresh = $want_unused - $fs_unused;
print("FS: $fs_size, Unused: $fs_unused, Threshold: $thresh -> " . ($fs_unused+$thresh));

print("Total: " . smallest_over_sz($dirmap, $thresh));
